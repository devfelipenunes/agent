import sys
import os
import json
import signal
import asyncio

# Suprime BrokenPipeError no Linux/macOS
try:
    signal.signal(signal.SIGPIPE, signal.SIG_DFL)
except AttributeError:
    pass

# Adiciona o diretório 'agent' ao path para permitir imports de 'src'
current_dir = os.path.dirname(os.path.abspath(__file__))
agent_dir = os.path.dirname(os.path.dirname(current_dir))
sys.path.append(agent_dir)

from typing import TypedDict, List, Dict, Any
from langgraph.graph import StateGraph, END
from langchain_ollama import ChatOllama
from langchain_openai import ChatOpenAI
from dotenv import load_dotenv

# Carrega variáveis de ambiente
load_dotenv(".env.elite")

class AgentState(TypedDict):
    query: str
    chat_history: List[Dict[str, str]]
    findings: List[str]
    artifact: str
    brief: str
    review_feedback: str
    current_status: str
    user_intent: str

def emit_event(event_type: str, agent: str, message: str):
    """Emits JSON-RPC style events to stdout for Rust to consume."""
    event = {
        "type": event_type,
        "agent": agent,
        "message": message
    }
    try:
        print(json.dumps(event), flush=True)
    except (BrokenPipeError, EOFError):
        sys.exit(0)

def emit_radio(agent: str, target: str, message: str):
    emit_event("radio", agent, f"-> {target}: {message}")

def emit_thought(agent: str, message: str):
    emit_event("thought", agent, message)

def emit_token(agent: str, token: str):
    emit_event("token", agent, token)

# Global LLM placeholders
llm_heavy = None
llm_chat = None

def init_models(mode: str):
    global llm_heavy, llm_chat
    
    # Heavy model is always Ollama for now (Gemma 26b)
    llm_heavy = ChatOllama(
        model="gemma4:26b",
        base_url="http://localhost:11434",
        temperature=0.1
    )
    
    if mode == "token":
        llm_chat = ChatOpenAI(
            model="gpt-4o",
            temperature=0.7
        )
        emit_event("system", "System", "Backend initialized in TOKEN mode (OpenAI)")
    else:
        llm_chat = llm_heavy
        emit_event("system", "System", "Backend initialized in LOCAL mode (Ollama)")

from src.tools.mcp_obsidian import ObsidianTool
from src.tools.arxiv import search_arxiv
from tavily import TavilyClient

# Inicializa ferramentas
vault_path = os.getenv("OBSIDIAN_VAULT_PATH", "/l/disk0/fnunes/obsidian/")
obsidian = ObsidianTool(vault_path)

tavily_key = os.getenv("TAVILY_API_KEY", "")
tavily = TavilyClient(api_key=tavily_key) if tavily_key else None

async def orchestrator_node(state: AgentState):
    query = state["query"]
    emit_event("status", "Aether-PM", "Analisando intenção...")
    emit_thought("Aether-PM", f"Processando entrada: '{query}'")
    
    prompt = f"""Você é o Aether-PM, o gerente de projetos da AetherMind. 
Análise: '{query}'.
Regras:
1. Se for cumprimento, responda curto.
2. Se for pesquisa/artigo, use a tag SQUAD_REQUIRED: RESEARCH.
3. Responda em Português."""
    
    full_response = ""
    emit_event("message_start", "Aether-PM", "")
    try:
        # Usando llm_chat (pode ser OpenAI ou Ollama dependendo do modo)
        for chunk in llm_chat.stream(prompt):
            content = chunk.content
            full_response += content
            emit_token("Aether-PM", content)
    except Exception as e:
        emit_event("error", "AI-Engine", str(e))
        return {"current_status": "error", "user_intent": "CHAT"}
    
    if "SQUAD_REQUIRED" in full_response:
        return {"current_status": "squad_needed", "user_intent": "RESEARCH"}
    else:
        return {"current_status": "chat_only", "user_intent": "CHAT"}

async def librarian_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Librarian", "Consultando Obsidian...")
    local_context = obsidian.search_notes(query)
    msg = "Notas locais encontradas." if "Nenhuma nota" not in local_context else "Sem notas locais."
    emit_event("message_start", "V3-Librarian", "")
    emit_event("token", "V3-Librarian", msg)
    return {"findings": [f"Obsidian: {local_context}"], "current_status": "librarian_done"}

async def scout_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Scout", "Minerando Web & ArXiv...")
    findings = []
    emit_radio("V3-Scout", "System", "Iniciando buscas externas...")
    
    try:
        arxiv_results = search_arxiv(query, max_results=3)
        findings.extend(arxiv_results)
    except Exception as e:
        findings.append(f"Erro ArXiv: {e}")

    if tavily:
        try:
            search_result = tavily.search(query=query, search_depth="advanced")
            web_findings = [f"Web: {r['title']} - {f['content'][:300]}..." for r in [search_result] for f in r.get('results', [])]
            findings.extend(web_findings)
        except Exception as e:
            findings.append(f"Erro Web: {e}")
    
    return {"findings": findings, "current_status": "scout_done"}

async def analyst_node(state: AgentState):
    emit_event("status", "V3-Analyst", "Sintetizando e salvando...")
    findings_text = "\n".join(state["findings"])
    
    if state["findings"]:
        try:
            title = state["query"].replace(" ", "_")[:30]
            obsidian.write_note(title, f"# Findings for {state['query']}\n\n{findings_text}")
            emit_radio("V3-Analyst", "Librarian", f"Salvo novo conhecimento em {title}.md")
        except Exception as e:
            emit_radio("V3-Analyst", "Librarian", f"Falha ao salvar nota: {e}")

    prompt = f"Crie um Briefing para artigo sobre: {state['query']} baseado em:\n{findings_text}"
    response = await llm_heavy.ainvoke(prompt)
    return {"brief": response.content, "current_status": "analyst_done"}

async def writer_node(state: AgentState):
    emit_event("status", "V8-Writer", "Escrevendo...")
    prompt = f"Escreva um artigo em Markdown para o briefing:\n{state['brief']}"
    
    full_art = ""
    emit_event("artifact_start", "V8-Writer", "")
    for chunk in llm_heavy.stream(prompt):
        content = chunk.content
        full_art += content
        emit_event("artifact_chunk", "V8-Writer", content)
    
    emit_event("artifact_full", "V8-Writer", full_art)
    return {"artifact": full_art, "current_status": "writer_done"}

async def critic_node(state: AgentState):
    emit_event("status", "V8-Critic", "Revisando...")
    return {"current_status": "approved"}

def route_intent(state: AgentState):
    return "research_path" if state.get("user_intent") == "RESEARCH" else "end"

# Build Graph
workflow = StateGraph(AgentState)
workflow.add_node("orchestrator", orchestrator_node)
workflow.add_node("librarian", librarian_node)
workflow.add_node("scout", scout_node)
workflow.add_node("analyst", analyst_node)
workflow.add_node("writer", writer_node)
workflow.add_node("critic", critic_node)
workflow.set_entry_point("orchestrator")
workflow.add_conditional_edges("orchestrator", route_intent, {"research_path": "librarian", "end": END})
workflow.add_edge("librarian", "scout")
workflow.add_edge("scout", "analyst")
workflow.add_edge("analyst", "writer")
workflow.add_edge("writer", "critic")
workflow.add_edge("critic", END)
app = workflow.compile()

async def main_loop():
    emit_event("system", "System", "AetherMind v4.2 Waiting for Config...")
    initialized = False
    
    while True:
        try:
            line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)
            if not line: break
            
            req = json.loads(line)
            
            if req.get("type") == "config":
                init_models(req.get("mode", "local"))
                initialized = True
                continue
            
            if not initialized:
                emit_event("error", "System", "Backend not initialized. Send config first.")
                continue

            query = req.get("query", "")
            if query:
                inputs = {
                    "query": query, "chat_history": [], "findings": [], 
                    "artifact": "", "brief": "", "review_feedback": "", 
                    "current_status": "", "user_intent": ""
                }
                await app.ainvoke(inputs)
                emit_event("system", "System", "Cycle complete")
        except (EOFError, KeyboardInterrupt):
            break
        except Exception as e:
            emit_event("error", "System", str(e))

if __name__ == "__main__":
    asyncio.run(main_loop())

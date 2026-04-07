import sys
import os
import json

# Adiciona o diretório 'agent' ao path para permitir imports de 'src'
current_dir = os.path.dirname(os.path.abspath(__file__))
agent_dir = os.path.dirname(os.path.dirname(current_dir))
sys.path.append(agent_dir)

from typing import TypedDict, List, Dict, Any
from langgraph.graph import StateGraph, END
from langchain_ollama import ChatOllama
from dotenv import load_dotenv

# Carrega variáveis de ambiente
load_dotenv(".env.elite")

class AgentState(TypedDict):
    query: str
    chat_history: List[Dict[str, str]]
    findings: List[str]
    artifact: str
    current_status: str

def emit_event(event_type: str, agent: str, message: str):
    """Emits JSON-RPC style events to stdout for Rust to consume."""
    event = {
        "type": event_type,
        "agent": agent,
        "message": message
    }
    try:
        print(json.dumps(event), flush=True)
    except BrokenPipeError:
        sys.exit(0)

def emit_radio(agent: str, target: str, message: str):
    emit_event("radio", agent, f"-> {target}: {message}")

def emit_thought(agent: str, message: str):
    emit_event("thought", agent, message)

# Configura o LLM Gemma 4
# Gemma 4 via Ollama v0.20.2
llm = ChatOllama(
    model="gemma4:26b",
    base_url="http://localhost:11434",
    temperature=0.1
)

from src.tools.mcp_obsidian import ObsidianTool
from tavily import TavilyClient

# Inicializa ferramentas
vault_path = os.getenv("OBSIDIAN_VAULT_PATH", "/l/disk0/fnunes/obsidian/")
obsidian = ObsidianTool(vault_path)

tavily_key = os.getenv("TAVILY_API_KEY", "")
tavily = TavilyClient(api_key=tavily_key) if tavily_key else None

def librarian_node(state: AgentState):
    query = state["query"]
    emit_thought("V3-Librarian", f"Iniciando busca por '{query}' no Obsidian Vault.")
    emit_radio("V3-Librarian", "Analyst", "Vou verificar se temos notas locais sobre isso.")
    
    local_context = obsidian.search_notes(query)
    
    if "Nenhuma nota encontrada" in local_context:
        emit_radio("V3-Librarian", "Scout", "Nada local. Sua vez de minerar a Web.")
        msg = "Não encontrei notas relacionadas no seu Obsidian."
    else:
        emit_radio("V3-Librarian", "Analyst", "Encontrei referências. Enviando para análise.")
        msg = "Encontrei referências locais interessantes."
    
    emit_event("message", "V3-Librarian", f"{msg}")
    return {"findings": [f"Obsidian: {local_context}"], "current_status": "librarian_done"}

def scout_node(state: AgentState):
    query = state["query"]
    emit_thought("V3-Scout", "Preparando motores de busca externa.")
    emit_radio("V3-Scout", "Squad", f"Minerando dados sobre '{query}' via Tavily.")
    
    if not tavily:
        emit_radio("V3-Scout", "Analyst", "Busca Web indisponível. Continuando com dados locais.")
        return {"findings": ["Busca Web desabilitada"], "current_status": "scout_done"}

    try:
        search_result = tavily.search(query=query, search_depth="advanced")
        findings = [f"Web: {r['title']} - {f['content'][:200]}..." for r in [search_result] for f in r.get('results', [])]
        emit_radio("V3-Scout", "Analyst", f"Encontrei {len(findings)} resultados na Web.")
    except Exception as e:
        emit_radio("V3-Scout", "System", f"Erro na busca: {e}")
        findings = [f"Erro na busca Web: {e}"]
    
    for finding in findings:
        emit_event("finding", "V3-Scout", finding)
    
    return {"findings": findings, "current_status": "scout_done"}

def analyst_node(state: AgentState):
    emit_thought("V3-Analyst", "Consolidando dados do Librarian e Scout.")
    emit_radio("V3-Analyst", "Architect", "Dados validados. Pode estruturar o rascunho.")
    return {"current_status": "analyst_done"}

def architect_node(state: AgentState):
    emit_thought("V8-Architect", "Gerando estrutura do artefato final.")
    emit_radio("V8-Architect", "User", "Aqui está o rascunho inicial.")
    
    findings = "\n".join(state.get("findings", []))
    artifact = f"# Draft: {state['query']}\n\nBaseado nas pesquisas:\n{findings}"
    emit_event("artifact_update", "V8-Architect", "Rascunho gerado.")
    
    return {"artifact": artifact, "current_status": "architect_done"}

# Constrói o Grafo de Estados
workflow = StateGraph(AgentState)
workflow.add_node("librarian", librarian_node)
workflow.add_node("scout", scout_node)
workflow.add_node("analyst", analyst_node)
workflow.add_node("architect", architect_node)

workflow.set_entry_point("librarian")
workflow.add_edge("librarian", "scout")
workflow.add_edge("scout", "analyst")
workflow.add_edge("analyst", "architect")
workflow.add_edge("architect", END)

app = workflow.compile()

def main_loop():
    emit_event("system", "System", "AetherMind Python Backend Started")
    for line in sys.stdin:
        try:
            req = json.loads(line)
            query = req.get("query", "")
            if query:
                inputs = {
                    "query": query,
                    "chat_history": [],
                    "findings": [],
                    "artifact": "",
                    "current_status": ""
                }
                # Executa o grafo síncrono para o exemplo, idealmente async
                app.invoke(inputs)
                emit_event("system", "System", "Workflow completed")
        except Exception as e:
            emit_event("error", "System", str(e))

if __name__ == "__main__":
    main_loop()

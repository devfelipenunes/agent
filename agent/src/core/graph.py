from typing import TypedDict, List, Dict, Any
from langgraph.graph import StateGraph, END
from langchain_ollama import ChatOllama
from dotenv import load_dotenv
import json
import sys
import os

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
    print(json.dumps(event), flush=True)

# Configura o LLM Gemma 4
# Gemma 4 via Ollama v0.20.2
llm = ChatOllama(
    model="gemma4:26b",
    base_url="http://localhost:11434",
    temperature=0.1
)

def librarian_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Librarian", "Buscando contexto local (Obsidian)...")
    
    # Simulação de busca no Cognee/Obsidian
    local_context = f"Contexto local para: {query}"
    
    emit_event("message", "V3-Librarian", f"Baseado nas suas notas, {query} é um tópico interessante. Iniciando pesquisa profunda...")
    
    return {"current_status": "librarian_done"}

def scout_node(state: AgentState):
    query = state["query"]
    emit_event("status", "V3-Scout", "Minerando a Web e GitHub...")
    
    # Simulação de busca
    finding = f"Artigo recente sobre {query} encontrado."
    emit_event("finding", "V3-Scout", finding)
    
    return {"findings": [finding], "current_status": "scout_done"}

def architect_node(state: AgentState):
    emit_event("status", "V8-Architect", "Estruturando o artefato final...")
    findings = "\n".join(state.get("findings", []))
    
    artifact = f"# Draft: {state['query']}\n\nBaseado nas pesquisas:\n{findings}"
    emit_event("artifact_update", "V8-Architect", "Rascunho gerado.")
    
    return {"artifact": artifact, "current_status": "architect_done"}

# Constrói o Grafo de Estados
workflow = StateGraph(AgentState)
workflow.add_node("librarian", librarian_node)
workflow.add_node("scout", scout_node)
workflow.add_node("architect", architect_node)

workflow.set_entry_point("librarian")
workflow.add_edge("librarian", "scout")
workflow.add_edge("scout", "architect")
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

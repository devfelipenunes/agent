import os
from typing import TypedDict, List
from langgraph.graph import StateGraph, END
from langchain_ollama import ChatOllama
from dotenv import load_dotenv

# Carrega variáveis de ambiente
load_dotenv(".env.elite")

class AgentState(TypedDict):
    query: str
    context: List[str]
    report: str
    thought: str

# Configura o LLM Gemma 4
# Gemma 4 via Ollama v0.20.2
llm = ChatOllama(
    model="gemma4:26b",
    base_url="http://localhost:11434",
    temperature=0.1
)

def research_node(state: AgentState):
    """Nodo de Pesquisa: Decide o que pesquisar."""
    query = state["query"]
    print(f"[*] Node (Research): Researching query: {query}")
    
    # Simulação de pensamento/decisão do LLM
    prompt = f"O usuário quer pesquisar sobre: {query}. O que você deve procurar primeiro? Responda de forma curta."
    try:
        response = llm.invoke(prompt)
        thought = response.content
    except Exception as e:
        thought = f"Erro ao acessar LLM: {e}"
        
    return {
        "thought": thought,
        "context": [f"Pesquisa inicial iniciada para: {query}"]
    }

def synthesize_node(state: AgentState):
    """Nodo de Síntese: Gera o relatório final."""
    query = state["query"]
    context = "\n".join(state["context"] if state.get("context") else [])
    thought = state.get("thought", "")
    
    print(f"[*] Node (Synthesize): Generating report for: {query}")
    
    prompt = f"""
    Objetivo: {query}
    Reflexão Anterior: {thought}
    Contexto: {context}
    
    Com base nas informações acima, gere um relatório técnico curto e estruturado em Português sobre este tópico.
    Foque nos pontos principais e próximos passos.
    """
    try:
        response = llm.invoke(prompt)
        report = response.content
    except Exception as e:
        report = f"Erro ao gerar relatório: {e}"
    
    return {"report": report}

# Constrói o Grafo de Estados
workflow = StateGraph(AgentState)
workflow.add_node("research", research_node)
workflow.add_node("synthesize", synthesize_node)

# Define as transições
workflow.set_entry_point("research")
workflow.add_edge("research", "synthesize")
workflow.add_edge("synthesize", END)

# Compila o grafo
app = workflow.compile()

if __name__ == "__main__":
    # Teste rápido se rodar o script diretamente
    test_query = "Impacto do Gemma 4 na produtividade de desenvolvedores"
    inputs = {"query": test_query, "context": [], "report": "", "thought": ""}
    
    print(f"--- Iniciando Workflow para: {test_query} ---")
    result = app.invoke(inputs)
    
    print("\n" + "="*50)
    print("FINAL REPORT")
    print("="*50)
    print(result["report"])
    print("="*50)

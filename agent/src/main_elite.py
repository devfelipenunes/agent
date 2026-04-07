import asyncio
import os
from dotenv import load_dotenv
from src.core.graph import app
from src.memory.cognee_setup import setup_memory
from src.tools.mcp_obsidian import ObsidianTool

# Carrega variáveis
load_dotenv(".env.elite")

async def run_elite_research(query: str):
    """Executa o pipeline completo de pesquisa elite."""
    print("\n" + "="*50)
    print(f"[*] INICIANDO PESQUISA ELITE: {query}")
    print("="*50)
    
    # 1. Setup da Memória
    print("[*] Configurando Cognee Memory Mesh...")
    await setup_memory()
    
    # 2. Inicializa Ferramentas
    vault_path = os.getenv("OBSIDIAN_VAULT_PATH", "/l/disk0/fnunes/obsidian/")
    obsidian = ObsidianTool(vault_path)
    
    # 3. Executa Busca no Obsidian (Pré-contexto)
    print(f"[*] Consultando conhecimento local (Obsidian) para: {query}...")
    local_context = obsidian.search_notes(query)
    print(f"[*] Contexto local obtido ({len(local_context)} caracteres).")
    
    # 4. Executa o Grafo
    print(f"[*] Ativando Orquestrador LangGraph (Gemma 4)...")
    inputs = {
        "query": query, 
        "context": [local_context], 
        "report": "", 
        "thought": ""
    }
    
    print("[*] Iniciando stream de eventos do LangGraph...")
    # Stream events for better visibility
    final_state = inputs
    async for event in app.astream(inputs):
        print(f"[DEBUG] Evento recebido: {list(event.keys())}")
        for node_name, state_update in event.items():
            print(f"--- Node {node_name} completed ---")
            final_state.update(state_update)
            
    print("\n" + "="*50)
    print("ELITE RESEARCH REPORT")
    print("="*50)
    print(final_state.get("report", "Erro ao gerar relatório."))
    print("="*50)
    
    # (Opcional) Salvar no Obsidian
    # Aqui poderíamos criar uma nova nota com o relatório.

if __name__ == "__main__":
    query = input("Digite o tópico de pesquisa: ") if not os.getenv("CI") else "ZK-Rollups"
    asyncio.run(run_elite_research(query))

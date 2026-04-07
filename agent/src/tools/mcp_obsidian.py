import os
import glob

class ObsidianTool:
    def __init__(self, vault_path):
        self.vault_path = vault_path
    
    def search_notes(self, query):
        """Busca simples por texto nas notas markdown do vault."""
        if not os.path.exists(self.vault_path):
            return f"Erro: Vault path não existe: {self.vault_path}"
            
        results = []
        # Busca recursiva em todos os arquivos .md
        for filename in glob.glob(os.path.join(self.vault_path, "**/*.md"), recursive=True):
            try:
                # Evita ler arquivos binários se o glob falhar ou houver resíduos
                if not filename.endswith(".md"):
                    continue
                    
                with open(filename, 'r', encoding='utf-8') as f:
                    content = f.read()
                    if query.lower() in content.lower():
                        results.append(f"File: {os.path.relpath(filename, self.vault_path)}\nContent: {content[:300]}...\n")
            except Exception as e:
                pass
        
        if not results:
            return f"Nenhuma nota encontrada no vault para: {query}"
        
        return "\n---\n".join(results[:5]) # Limita a 5 resultados para o contexto

if __name__ == "__main__":
    # Teste rápido
    vault = "/l/disk0/fnunes/obsidian/"
    tool = ObsidianTool(vault)
    print(tool.search_notes("ZK-Rollups"))

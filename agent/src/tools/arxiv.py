import urllib.request
import urllib.parse
import xml.etree.ElementTree as ET

def search_arxiv(query: str, max_results: int = 5) -> list[str]:
    """
    Realiza uma busca na API do arXiv.
    Retorna uma lista de strings com os resultados formatados.
    """
    base_url = 'http://export.arxiv.org/api/query?'
    
    # Substitui espaços e formata a query para o arXiv se necessário
    formatted_query = query.replace(" ", "+")
    # Para ser seguro, adicionamos all: antes dos termos, mas urllib.parse
    # já ajuda a codificar. O ideal é que a query chegue já limpa.
    encoded_query = urllib.parse.quote(f"all:{formatted_query}")
    
    url = f"{base_url}search_query={encoded_query}&start=0&max_results={max_results}"
    findings = []
    
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0 AetherMind/1.0'})
        response = urllib.request.urlopen(req)
        xml_data = response.read()
        
        root = ET.fromstring(xml_data)
        ns = {'atom': 'http://www.w3.org/2005/Atom'}
        entries = root.findall('atom:entry', ns)
        
        if not entries:
            return []
            
        for i, entry in enumerate(entries, 1):
            title = entry.find('atom:title', ns).text.replace('\n', ' ').strip()
            link = entry.find('atom:id', ns).text
            published = entry.find('atom:published', ns).text[:10]
            summary = entry.find('atom:summary', ns).text.replace('\n', ' ').strip()
            
            findings.append(f"ArXiv: {title} ({published}) [{link}] - {summary[:400]}...")
            
        return findings
    except Exception as e:
        return [f"Erro na busca ArXiv: {e}"]

if __name__ == "__main__":
    # Test
    res = search_arxiv("blockchain interoperability")
    for r in res:
        print(r)

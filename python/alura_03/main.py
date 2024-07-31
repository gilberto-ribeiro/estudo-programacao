from modelos.restaurante import Restaurante
from modelos.cardapio.prato import Prato
from modelos.cardapio.bebida import Bebida
from modelos.cardapio.sobremesa import Sobremesa
import requests
import json

url = "https://guilhermeonrails.github.io/api-restaurantes/restaurantes.json"

def importar_restaurantes(url: str) -> dict:
    response = requests.get(url)
    if response.status_code == 200:
        dados_json = response.json()
        dados_restaurante = {}
        for item in dados_json:
            nome_do_restaurante = item["Company"]
            if nome_do_restaurante not in dados_restaurante:
                dados_restaurante[nome_do_restaurante] = []
            dados_restaurante[nome_do_restaurante].append({
                "nome": item["Item"],
                "preco": item["price"],
                "descricao": item["description"]
            })
        return dados_restaurante
    else:
        print(f"Falha ao importar JSON. Código: {response.status_code}.")

def exportar_json_restaurantes(dados_restaurantes: dict) -> None:
    for restaurante, dados in dados_restaurantes.items():
        nome_do_arquivo = f"restaurantes_json/{restaurante}.json"
        with open(nome_do_arquivo, 'w') as arquivo_json:
            json.dump(dados, arquivo_json, indent=4)


def main():
    dados_restaurantes = importar_restaurantes(url)
    exportar_json_restaurantes(dados_restaurantes)


if __name__ == "__main__":
    main()
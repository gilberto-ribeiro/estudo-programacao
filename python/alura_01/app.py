import os

restaurantes = [
    {"nome": "Batagini", "categoria": "Pizza", "ativo": True},
    {"nome": "Burguer King", "categoria": "Fast Food", "ativo": True},
    {"nome": "McDonald's", "categoria": "Fast Food", "ativo": False},
    {"nome": "Outback", "categoria": "Restaurante", "ativo": True},
    {"nome": "Subway", "categoria": "Fast Food", "ativo": False}
]

def exibir_nome_do_programa():
    print(
        """
░██████╗░█████╗░██████╗░░█████╗░██████╗░  ███████╗██╗░░██╗██████╗░██████╗░███████╗░██████╗░██████╗
██╔════╝██╔══██╗██╔══██╗██╔══██╗██╔══██╗  ██╔════╝╚██╗██╔╝██╔══██╗██╔══██╗██╔════╝██╔════╝██╔════╝
╚█████╗░███████║██████╦╝██║░░██║██████╔╝  █████╗░░░╚███╔╝░██████╔╝██████╔╝█████╗░░╚█████╗░╚█████╗░
░╚═══██╗██╔══██║██╔══██╗██║░░██║██╔══██╗  ██╔══╝░░░██╔██╗░██╔═══╝░██╔══██╗██╔══╝░░░╚═══██╗░╚═══██╗
██████╔╝██║░░██║██████╦╝╚█████╔╝██║░░██║  ███████╗██╔╝╚██╗██║░░░░░██║░░██║███████╗██████╔╝██████╔╝
╚═════╝░╚═╝░░╚═╝╚═════╝░░╚════╝░╚═╝░░╚═╝  ╚══════╝╚═╝░░╚═╝╚═╝░░░░░╚═╝░░╚═╝╚══════╝╚═════╝░╚═════╝░
"""
    )

def exibir_opcoes():
    print(
        """1. Cadastrar restaurante.
2. Listar restaurantes.
3. Alternar estado do restaurante.
4. Sair.
"""
    )

def retornar_ao_menu_principal():
    input("\nPressione enter para retornar ao menu principal. ")
    main()

def opcao_invalida():
    print("Opção inválida.")
    retornar_ao_menu_principal()

def exibir_subtitulo(subtitulo):
    os.system("clear")
    print(subtitulo)

def finalizar_app():
    exibir_subtitulo(
        """
█▀▀ █ █▄░█ ▄▀█ █░░ █ ▀█ ▄▀█ █▄░█ █▀▄ █▀█   ▄▀█ █▀█ █▀█ ░
█▀░ █ █░▀█ █▀█ █▄▄ █ █▄ █▀█ █░▀█ █▄▀ █▄█   █▀█ █▀▀ █▀▀ ▄"""
    )

def cadastrar_restaurante():
    exibir_subtitulo(
        """
█▀▀ ▄▀█ █▀▄ ▄▀█ █▀ ▀█▀ █▀█ ▄▀█ █▀▄▀█ █▀▀ █▄░█ ▀█▀ █▀█   █▀▄ █▀▀   █▀█ █▀▀ █▀ ▀█▀ ▄▀█ █░█ █▀█ ▄▀█ █▄░█ ▀█▀ █▀▀
█▄▄ █▀█ █▄▀ █▀█ ▄█ ░█░ █▀▄ █▀█ █░▀░█ ██▄ █░▀█ ░█░ █▄█   █▄▀ ██▄   █▀▄ ██▄ ▄█ ░█░ █▀█ █▄█ █▀▄ █▀█ █░▀█ ░█░ ██▄
"""
    )
    nome_restaurante = input("Insira o nome do novo restaurante: ")
    caegoria_restaurante = input("Insira a categoria do novo restaurante: ")
    novo_restaurante = {"nome": nome_restaurante, "categoria": caegoria_restaurante, "ativo": False}
    restaurantes.append(novo_restaurante)
    print(f"O restaurante {novo_restaurante['nome']} foi cadastrado com sucesso!")
    retornar_ao_menu_principal()

def listar_restaurantes():
    exibir_subtitulo(
        """
█░░ █ █▀ ▀█▀ ▄▀█   █▀▄ █▀▀   █▀█ █▀▀ █▀ ▀█▀ ▄▀█ █░█ █▀█ ▄▀█ █▄░█ ▀█▀ █▀▀ █▀
█▄▄ █ ▄█ ░█░ █▀█   █▄▀ ██▄   █▀▄ ██▄ ▄█ ░█░ █▀█ █▄█ █▀▄ █▀█ █░▀█ ░█░ ██▄ ▄█
"""
    )
    espaco = 20
    print(f"{' '.rjust(4)}{'Restaurante'.ljust(espaco)} | {'Categoria'.ljust(espaco)} | Estado")
    for i, restaurante in enumerate(restaurantes):
        print(f"{str(i+1).rjust(2)}. {restaurante['nome'].ljust(espaco)} | {restaurante['categoria'].ljust(espaco)} | {'Ativo' if restaurante['ativo'] else 'Inativo'}")
    retornar_ao_menu_principal()

def alternar_estado_do_restaurante():
    exibir_subtitulo(
        """
▄▀█ █░░ ▀█▀ █▀▀ █▀█ █▄░█ ▄▀█ █▄░█ █▀▄ █▀█   █▀▀ █▀ ▀█▀ ▄▀█ █▀▄ █▀█   █▀▄ █▀█   █▀█ █▀▀ █▀ ▀█▀ ▄▀█ █░█ █▀█ ▄▀█ █▄░█ ▀█▀ █▀▀
█▀█ █▄▄ ░█░ ██▄ █▀▄ █░▀█ █▀█ █░▀█ █▄▀ █▄█   ██▄ ▄█ ░█░ █▀█ █▄▀ █▄█   █▄▀ █▄█   █▀▄ ██▄ ▄█ ░█░ █▀█ █▄█ █▀▄ █▀█ █░▀█ ░█░ ██▄
"""
    )
    nome_restaurante = input("Digite o nome do restaurante que deseja alternar o estado: ")
    restaurante_encontrado = False

    for restaurante in restaurantes:
        if nome_restaurante == restaurante["nome"]:
            restaurante["ativo"] = not restaurante["ativo"]
            restaurante_encontrado = True
            novo_estado = "ativado" if restaurante["ativo"] else "desativado"

    if restaurante_encontrado:
        print(f"Restaurante {nome_restaurante} {novo_estado} com sucesso!")
    else:
        print(f"Restaurante {nome_restaurante} não foi encontrado na base de dados.")

    retornar_ao_menu_principal()

def escolher_opcao():
    try:
        opcao_escolhida = int(input("Escolha uma opção: "))
        match opcao_escolhida:
            case 1:
                cadastrar_restaurante()
            case 2:
                listar_restaurantes()
            case 3:
                alternar_estado_do_restaurante()
            case 4:
                finalizar_app()
            case _:
                opcao_invalida()
    except:
        opcao_invalida()

def main():
    os.system("clear")
    exibir_nome_do_programa()
    exibir_opcoes()
    escolher_opcao()

if __name__ == "__main__":
    main()
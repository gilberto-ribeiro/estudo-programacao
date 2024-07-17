from modelo.restaurante import Restaurante

restaurante_1 = Restaurante("Burger King", "Fast Food")
restaurante_2 = Restaurante("Di Caprio", "Pizzaria")
restaurante_3 = Restaurante("Bardana", "Restaurante")

def main():
    restaurante_1.alternar_estado()
    restaurante_3.alternar_estado()
    Restaurante.listar_restaurantes()
    print()
    restaurante_1.receber_avaliacao("Gilberto", 4)
    restaurante_2.receber_avaliacao("Giovanna", 3)
    restaurante_1.receber_avaliacao("Angela", 5)
    restaurante_1.listar_avaliacoes()
    restaurante_2.receber_avaliacao("Maria", 5)
    print()
    Restaurante.listar_restaurantes()

    print(help(Restaurante.listar_restaurantes()))

if __name__ == "__main__":
    main()
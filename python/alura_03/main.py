from modelos.restaurante import Restaurante
from modelos.cardapio.prato import Prato
from modelos.cardapio.bebida import Bebida
from modelos.cardapio.sobremesa import Sobremesa

restaurante_1 = Restaurante("Burger King", "Fast Food")
# restaurante_2 = Restaurante("Di Caprio", "Pizzaria")
# restaurante_3 = Restaurante("Bardana", "Restaurante")

bebida_1 = Bebida("Suco de Melancia", 10.00, "500ml")
bebida_1.aplicar_desconto()
prato_1 = Prato("Pãozinho", 2.00, "Pão de fermentação natural, quentinho.")
prato_1.aplicar_desconto()
bebida_2 = Bebida("Suco de Maracujá", 5.00, "400ml")
bebida_2.promocao()
prato_1.promocao()
sobremesa_1 = Sobremesa("Chocolate", 3.00, "Chocolate branco", "grande")
sobremesa_2 = Sobremesa("Mousse de maracujá", 4.00, "Cremosinho", "médio")
sobremesa_1.aplicar_desconto()


def main():
    restaurante_1.adicionar_ao_cardapio(prato_1)
    restaurante_1.adicionar_ao_cardapio(bebida_1)
    restaurante_1.adicionar_ao_cardapio(bebida_2)
    restaurante_1.adicionar_ao_cardapio(sobremesa_1)
    restaurante_1.adicionar_ao_cardapio(sobremesa_2)
    restaurante_1.mostrar_cardapio()


if __name__ == "__main__":
    main()
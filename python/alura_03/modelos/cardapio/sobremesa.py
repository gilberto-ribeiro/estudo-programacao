from modelos.cardapio.item_cardapio import ItemCardapio

class Sobremesa(ItemCardapio):
    def __init__(self, nome, preco, descricao, tamanho):
        super().__init__(nome, preco)
        self._descricao = descricao
        self._tamanho = tamanho

    def aplicar_desconto(self):
        self._preco -= (self._preco * 0.10)
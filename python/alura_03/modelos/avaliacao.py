class Avaliacao:

    def __init__(self, cliente, nota):
        self._cliente = cliente.title()
        self._nota = float(nota)

    @property
    def cliente(self):
        return self._cliente

    @property
    def nota(self):
        return self._nota
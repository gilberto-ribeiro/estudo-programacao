from modelo.avaliacao import Avaliacao


class Restaurante:
    """
    Classe restaurante, cubrindo características como nome, categoria, estado e notas.
    """

    restaurantes = []

    def __init__(self, nome, categoria):
        """
        Inicializa a instância Restaurante

        Parâmetros:
        - nome (str): Nome do restaurante.
        - categoria (str): Categoria do restaurante.

        Outputs:
        - Estado do restaurante como False (bool).
        - Acrescenta a instância na lista de classe restaurantes.
        """
        self._nome = nome.title()
        self._categoria = categoria.upper()
        self._ativo = False
        self._avaliacoes = []
        Restaurante.restaurantes.append(self)

    def __str__(self):
        """
        Retorna a representação de String do restaurante.
        """
        return f"""Restaurante: {self._nome}
Categoria: {self._categoria}
Estado: {self.ativo}"""

    @property
    def ativo(self):
        """
        Retorna a representação do estado do restaurante (se Ativo ou Inativo).
        """
        return "🟢 Ativo" if self._ativo else "🔴 Inativo"

    def alternar_estado(self):
        """
        Alterna o estado do restaurante de True (Ativo) para False (Inativo) e vice-versa
        """
        self._ativo = not self._ativo

    def receber_avaliacao(self, cliente, nota):
        """
        Recebe a nota de avaliação de um cliente de um cliente.
        A nota deve ser positiva e menor ou igual a 5.

        Parâmetros:
        - cliente (str): Nome do cliente que avaliou.
        - nota (float): Nota dada pelo cliente.

        Outputs:
        - Acrescenta a avaliação poe meio da classe Avaliacao ao conjunto de avaliações do restaurante.
        """
        if 0 <= nota <= 5:
            avaliacao = Avaliacao(cliente, nota)
            self._avaliacoes.append(avaliacao)

    def listar_avaliacoes(self):
        """
        Lista todas as avaliações (se houver) do restaurante, discriminando cliente e sua respectiva nota.
        """
        if self._avaliacoes:
            print(f"Avaliações de {self._nome} ({self.media_avaliacoes})".upper())
            print(f"{'Cliente'.ljust(25)} | {'Nota'}")
            for avaliacao in self._avaliacoes:
                print(f"{avaliacao.cliente.ljust(25)} | {avaliacao.nota}")
        else: 
            print(f"Não há avaliações para {self._nome}")

    @property
    def media_avaliacoes(self):
        """
        Retorna a média de avaliações do restaurante (se houver).
        """
        if self._avaliacoes:
            notas = [avaliacao.nota for avaliacao in self._avaliacoes]
            return round(sum(notas)/len(notas), 1)
        else:
            return "N/A"
    
    @classmethod
    def listar_restaurantes(cls):
        """
        Lista todos os restaurantes instânciados, discriminando:
        - Nome;
        - Categoria;
        - Status;
        - Nota média (se houver).
        """
        cabecalho = f"{'Restaurante'.ljust(25)} | {'Categoria'.ljust(25)} | {'Status'.ljust(26)} | Nota"
        print(cabecalho)
        for restaurante in cls.restaurantes:
            print(f"{restaurante._nome.ljust(25)} | {restaurante._categoria.ljust(25)} | {restaurante.ativo.ljust(25)} | {restaurante.media_avaliacoes}")

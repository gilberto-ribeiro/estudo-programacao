class Veiculo:
    def __init__(self, marca: str, modelo: str) -> None:
        self.marca = marca
        self.modelo = modelo
        self._ligado = False
        
    @property
    def ligado(self):
        match self._ligado:
            case True:
                return "Ligado"
            case False:
                return "Desligado"

    def __str__(self):
        return f"Veículo {self.modelo} da marca {self.marca}. {self.ligado}."


class Carro(Veiculo):
    def __init__(self, marca: str, modelo: str, portas: int) -> None:
        super().__init__(marca, modelo)
        self.portas = portas


class Moto(Veiculo):
    def __init__(self, marca: str, modelo: str, tipo: str) -> None:
        super().__init__(marca, modelo)
        self.tipo = tipo

    def __str__(self):
        return f"{super().__str__()} Tipo {self.tipo}."

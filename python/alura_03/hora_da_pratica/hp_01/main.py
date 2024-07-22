from veiculos import Carro, Moto

carro_1 = Carro("Bentley", "Continental", 4)
carro_2 = Carro("Chevrolet", "Astra", 4)
carro_3 = Carro("VolksWagen", "Fusca", 2)
carro_1.ligar()
moto_1 = Moto("Kawasaki", "Ninja 400", "Esportiva")
moto_2 = Moto("Suzuki", "GSX-R 1000 R", "Esportiva")
moto_2.ligar()
moto_3 = Moto("Harley-Davidson", "Iron 833", "Clássica")
def main():
    print(carro_1)
    print(carro_2)
    print(carro_3)
    print(moto_1)
    print(moto_2)
    print(moto_3)

if __name__ == "__main__":
    main()
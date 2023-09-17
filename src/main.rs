


fn main() {
    zadanie1();
   
}

fn zadanie2(){

}

fn zadanie1(){

    #[derive(Debug, Copy, Clone)]
    enum Smak{
        slodki,
        kwasny,
        gorzki
    }


    #[derive(Debug)]
    struct Gruszka{
        odmiana: String,
        kolor: String,
        waga: f64,
        smak: Smak
    }

    impl Gruszka{
        fn construct(odmiana: String, kolor: String, waga: f64, smak: Smak) -> Gruszka{
            Gruszka{
                odmiana: odmiana,
                kolor: kolor,
                waga: waga,
                smak: smak
            }
        }

        fn set_odmiana(&self, odmiana: String) -> Gruszka{
            Gruszka{
                odmiana: odmiana,
                kolor: self.kolor.clone(),
                waga: self.waga,
                smak: self.smak
            }
        }

        fn set_odmiana_mutable(&mut self, odmiana: String){
            self.odmiana = odmiana
        }

        fn get_odmiana(&self) -> String{
            self.odmiana.clone()
        }
    }


    let gruszka = Gruszka{
        odmiana: String::from("casees"),
        kolor: String::from("niebieski"),
        waga: 3.23,
        smak: Smak::slodki
    };

    println!("gruszka: {:?}", gruszka);

    let gruszka2 = Gruszka::construct(
        String::from("casees"),
        String::from("niebieski"),
        3.23,
        Smak::slodki
    );

    println!("gruszka2: {:?}", gruszka2);

    let gruszka3 = gruszka2.set_odmiana("laaaa".to_string());
    println!("gruszka3: {:?}", gruszka3);
    let odmiana = gruszka3.get_odmiana();
    println!("odmiana: {odmiana}");



}
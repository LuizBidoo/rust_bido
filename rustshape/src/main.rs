enum Shape{ //Uma enum que tem todos os tipos de formas enumerados com os tipos de  dados que serão passados
	Circulo(f64),
	Quadrado(f64),
	Triangulo(f64,f64),
	Retangulo(f64, f64),
	Nenhum
}

Shape::*

fn calcula_area(forma: Shape){
	match forma{
		Circulo(x) => println!("Área do círculo: {:?}", (x * x) * (std::f64::consts::PI));
		Quadrado(x) => println!("Área do quadrado: {:?}", x * x);
		Triangulo(x,y) => println!("Área do triângulo: {:?}", (x*y)/2.0);
		Retangulo(x,y) => println!("Área do retângulo: {:?}", x*y);
		_ => println!("Valor não aceito!");

}
fn main() {
	let circulo_medidas = Circulo(5.0); // nao esquecer de fazer Shape::* para não precisar fazer isso no corpo do código
	let quadrado_medidas = Quadrado(4.0);
	let triangulo_medidas = Triangulo(3.0, 4.0);
	let retangulo_medidas = Retangulo(5.0, 4.0);

	calcular_area(circulo);
	calcular_area(quadrado);
	calcular_area(triangulo);
	calcular_area(retangulo); 
}

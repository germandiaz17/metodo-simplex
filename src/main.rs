fn main() {
    //Tableu inicial
    // Columnas: x, y, s1, s2, RHS

    let mut tableu: Vec<Vec<f64>> = vec![
        vec![2.0, 3.0, 1.0, 0.0, 12.0], // restriccion 1
        vec![1.0, 2.0, 0.0, 1.0, 8.0], // restriccion 2
        vec![-40.0, -50.0, 0.0, 0.0, 0.0], // función objetivo z
    ];

    let num_vars: i32 = 4; // x, y, s1, s2
    let nombres = ["x", "y", "s1", "s2"];
    let mut base = vec!["s1", "s2"]; //variables basicas iniciales

    println!(" === MÉTODO SIMPLEX ===\n");

    imprimir_tableu(&tableu, &base, &nombres)
}


fn imprimir_tableu(tableu: &Vec<Vec<f64>>, base: &Vec<&str>, nombres: &[&str]) {
    println!("Base | {} | RHS", nombres.join("             "));
    println!("{}", "-".repeat(50));
    for (i, fila) in tableu.iter().enumerate() {
        let etiqueta = if i < base.len() { base[i] } else { "  Z  " };
        let valores: Vec<String> = fila.iter().map(|v| format!("{:6.2}", v)).collect();
        println!("{:4} | {}", etiqueta, valores.join("     "));
    }
    println!();
}

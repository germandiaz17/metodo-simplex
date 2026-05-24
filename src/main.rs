mod grafico;

fn main() {
    let mut tableau: Vec<Vec<f64>> = vec![
        vec![2.0,   3.0,  1.0, 0.0, 12.0],
        vec![1.0,   2.0,  0.0, 1.0,  8.0],
        vec![-40.0, -50.0, 0.0, 0.0,  0.0],
    ];

    let nombres = ["x", "y", "s1", "s2"];
    let mut base = vec!["s1", "s2"];

    println!("=== MÉTODO SIMPLEX ===\n");
    println!("Tableau inicial:");
    imprimir_tableau(&tableau, &base, &nombres);

    let mut iteracion = 1;

    loop {
        // buscar columna pivote
        let col = match encontrar_columna_pivote(&tableau) {
            Some(c) => c,
            None => {
                println!("✓ Solución óptima encontrada.");
                break;
            }
        };

        // buscar fila pivote
        let fila = match encontrar_fila_pivote(&tableau, col) {
            Some(f) => f,
            None => {
                println!("El problema no tiene solución acotada.");
                break;
            }
        };

        println!("Iteración {}:", iteracion);
        println!("  Columna pivote: {} ({})", col, nombres[col]);
        println!("  Fila pivote: {} ({})", fila, base[fila]);

        // actualizar base
        base[fila] = nombres[col];

        // pivotear
        pivotear(&mut tableau, fila, col);

        println!("Tableau después de pivotear:");
        imprimir_tableau(&tableau, &base, &nombres);

        iteracion += 1;
    }

    // imprimir solución final
    println!("=== SOLUCIÓN FINAL ===");
    println!("Valor óptimo Z = {:.2}", tableau.last().unwrap().last().unwrap());
    for (i, var) in base.iter().enumerate() {
        println!("{} = {:.2}", var, tableau[i].last().unwrap());
    }

    grafico::graficar("Programación Lineal - Método Gráfico").unwrap();
}


fn imprimir_tableau(tableau: &Vec<Vec<f64>>, base: &Vec<&str>, nombres: &[&str]) {
    println!("Base | {} | RHS", nombres.join("             "));
    println!("{}", "-".repeat(50));
    for (i, fila) in tableau.iter().enumerate() {
        let etiqueta = if i < base.len() { base[i] } else { "  Z  " };
        let valores: Vec<String> = fila.iter().map(|v| format!("{:6.2}", v)).collect();
        println!("{:4} | {}", etiqueta, valores.join("     "));
    }
    println!();
}


fn encontrar_columna_pivote(tableau: &Vec<Vec<f64>>) -> Option<usize> {
    let fila_z = tableau.last().unwrap();
    let mut min_val = 0.0;
    let mut col_pivote = None;

    for (i, &val) in fila_z.iter().enumerate() {
        // ignorar la última columna (RHS)
        if i < fila_z.len() - 1 && val < min_val {
            min_val = val;
            col_pivote = Some(i);
        }
    }
    col_pivote
}


fn encontrar_fila_pivote(tableau: &Vec<Vec<f64>>, col: usize) -> Option<usize> {
    let num_restricciones = tableau.len() - 1; // excluir fila Z
    let mut min_ratio = f64::INFINITY;
    let mut fila_pivote = None;

    for i in 0..num_restricciones {
        let elemento = tableau[i][col];
        if elemento > 0.0 {
            let ratio = tableau[i].last().unwrap() / elemento;
            if ratio < min_ratio {
                min_ratio = ratio;
                fila_pivote = Some(i);
            }
        }
    }
    fila_pivote
}


fn pivotear(tableau: &mut Vec<Vec<f64>>, fila: usize, col: usize) {
    let num_filas = tableau.len();
    let num_cols = tableau[0].len();

    // hacer el elemento pivote = 1
    let pivote = tableau[fila][col];
    for j in 0..num_cols {
        tableau[fila][j] /= pivote;
    }

    // hacer ceros en el resto de la columna pivote
    for i in 0..num_filas {
        if i != fila {
            let factor = tableau[i][col];
            for j in 0..num_cols {
                tableau[i][j] -= factor * tableau[fila][j];
            }
        }
    }
}

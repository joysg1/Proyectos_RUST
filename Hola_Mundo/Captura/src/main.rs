use std::process::Command;

fn main() {
    banner();
    system_info();
    rust_info();
    check_manjaro();
}

fn banner() {
    let banner = r#"
    ███╗   ███╗ █████╗ ███╗   ██╗     ██╗ █████╗ ██████╗  ██████╗ 
    ████╗ ████║██╔══██╗████╗  ██║     ██║██╔══██╗██╔══██╗██╔═══██╗
    ██╔████╔██║███████║██╔██╗ ██║     ██║███████║██████╔╝██║   ██║
    ██║╚██╔╝██║██╔══██║██║╚██╗██║██   ██║██╔══██║██╔══██╗██║   ██║
    ██║ ╚═╝ ██║██║  ██║██║ ╚████║╚█████╔╝██║  ██║██║  ██║╚██████╔╝
    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ 
    "#;
    
    println!("{}", banner);
    println!("✨ Hola Mundo en Rust para Manjaro Linux ✨\n");
}

fn system_info() {
    println!("=== INFORMACIÓN DEL SISTEMA ===");
    
    // Detectar si estamos en Manjaro (aproximación)
    if let Ok(output) = Command::new("cat").arg("/etc/os-release").output() {
        if let Ok(os_info) = String::from_utf8(output.stdout) {
            if os_info.contains("Manjaro") {
                println!("✅ Detectado: Manjaro Linux");
                
                // Extraer versión
                for line in os_info.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        let pretty_name = line.replace("PRETTY_NAME=", "").replace("\"", "");
                        println!("📋 Versión: {}", pretty_name);
                    }
                }
            } else {
                println!("⚠️  No se detectó Manjaro específicamente");
                println!("📋 Sistema: Linux (posiblemente otra distro)");
            }
        }
    } else {
        println!("⚠️  No se pudo leer /etc/os-release");
    }
    
    println!("💾 Arquitectura: {}-{}", 
             std::env::consts::OS, 
             std::env::consts::ARCH);
}

fn rust_info() {
    println!("\n=== INFORMACIÓN DE RUST ===");
    println!("🦀 Versión: Rust {}", env!("CARGO_PKG_VERSION"));
    println!("📦 Cargo: Gestor de paquetes integrado");
    println!("🎯 Edición: 2021");
    
    // Features disponibles
    #[cfg(feature = "default")]
    println!("⚙️  Features: default");
}

fn check_manjaro() {
    println!("\n=== VERIFICACIÓN MANJARO ===");
    
    // Verificar comandos comunes de Manjaro
    let commands = [
        ("pacman", "✅ Gestor de paquetes Arch/Manjaro"),
        ("pamac", "✅ GUI para gestión de paquetes"),
        ("mhwd", "✅ Driver manager de Manjaro"),
    ];
    
    for (cmd, desc) in commands.iter() {
        match Command::new("which").arg(cmd).output() {
            Ok(output) if output.status.success() => {
                println!("{}", desc);
            },
            _ => println!("❌ {} no encontrado", cmd),
        }
    }
    
    // Verificar si estamos en Manjaro específicamente
    let manjaro_check = Command::new("uname")
        .arg("-r")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout).contains("MANJARO")
        })
        .unwrap_or(false);
    
    if manjaro_check {
        println!("🎯 Kernel: Manjaro personalizado");
    }
    
    println!("\n🎉 ¡Hola Mundo ejecutándose correctamente!");
    println!("🚀 Listo para desarrollar aplicaciones nativas.");
}

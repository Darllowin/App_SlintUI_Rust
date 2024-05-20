slint::include_modules!();
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
//use slint::ComponentHandle;
mod win1 {
    include!(env!("SLINT_INCLUDE_SUB"));
}
mod win2 {
    include!(env!("SLINT_INCLUDE_APP"));
}
use crate::win1::LoginWindow as OtherLoginWindow;
use crate::win2::AppWindow as OtherAppWindow;

fn main() -> Result<(), slint::PlatformError> {
    let ui1 = OtherLoginWindow::new()?;
    let ui = OtherAppWindow::new()?;

    // Попытка открыть соединение с базой данных
    let conn_result = Connection::open("mydatabase.db");
    // Обработка результата с помощью match
    match conn_result {
        Ok(conn) => {
            println!("Соединение с базой данных установлено успешно.");
            let conn = Arc::new(Mutex::new(conn)); // Обернуть conn в Arc<Mutex<Connection>>
            let conn_clone = Arc::clone(&conn); // Создать клон для передачи в замыкания
            let ui1_handle = ui1.as_weak();
            ui1.on_login_requested(move |email, password| {
                let ui1 = ui1_handle.unwrap();
                let conn = conn_clone.lock().unwrap(); // Получить доступ к conn внутри замыкания
                let string_email = String::from(email);
                let string_password = String::from(password);
                match check_credentials(&conn, &string_email, &string_password) {
                    Ok(true) => {
                        // Пользователь авторизован
                        println!("Login successful for user {}",  string_email);
                        ui.run();
                        ui1.hide();
                        
                    },
                    Ok(false) => {
                        // Неправильная пара логин-пароль
                        println!("Login failed for user {}, {}",  string_email, &string_password);
                        //println!("{}, {}", username, password)
                    },
                    Err(err) => {
                        // Произошла ошибка при проверке
                        println!("Error checking credentials: {}", err);
                    },
                }
            });
            let ui11_handle = ui1.as_weak();
            let conn_clone = Arc::clone(&conn); // Создать клон для передачи в замыкания
            ui1.on_registration_requested(move |name, surname, email, password, seccond_password | {
                let conn = conn_clone.lock().unwrap(); // Получить доступ к conn внутри замыкания
                let ui1 = ui11_handle.unwrap();
                match create_user(&conn, &name, &surname, &email, &password, &seccond_password) {
                    Ok(_) => {
                        // Пользователь успешно зарегистрирован
                        println!("User {} registered successfully", name);
                        
                    },
                    Err(err) => {
                        // Произошла ошибка при регистрации
                        println!("Error registering user {}: {}", name, err);
                    },
                }
            });
        }
        Err(err) => {
            eprintln!("Ошибка при открытии базы данных: {}", err);
            // Дальнейшие действия при ошибке
        }
    }
    ui1.run();
    Ok(())
}

fn create_user(conn: &Connection, name: &str, surname: &str, email: &str, password: &str, seccond_password: &str,) -> Result<()> {
    if password == seccond_password {
        conn.execute(
            "INSERT INTO Users (name, surname, email, password) VALUES (?, ?, ?, ?)",
            &[name, surname, email, password],
        )?;
    } else {
        println!("Пароли не совпадают");
    }
    Ok(())
}

fn check_credentials(conn: &Connection, email: &str, password: &str) -> Result<bool> {
    let mut stmt: rusqlite::Statement = conn.prepare("SELECT * FROM Users WHERE email = ? AND password = ?")?;
    let user_exists = stmt.exists(&[email, password])?;

    Ok(user_exists)
}


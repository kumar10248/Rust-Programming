mod add;
mod student_record;
use student_record::{search_student_rollno, load_students,Student};

use std::fs::File;
use std::io::{BufRead, Write};


struct Book{
    book_id: u32,
    title: String,
    author: String,
    publication_year: u16,
    issued_to:Option<String>,
    status: BookStatus,
}

impl Book {
    // Constructor for a new, available book
    pub fn new(book_id: u32, title: String, author: String, publication_year: u16) -> Self {
        Book {
            book_id,
            title,
            author,
            publication_year,
            issued_to: None, // Always defaults to None for new books
            status: BookStatus::Available,
        }
    }
}
#[derive(Debug)]
enum BookStatus{
    Available,
    Issued,
}

impl Book {
    fn display(&self) {

        println!(
            "{:<10} {:<30} {:<30} {:<12}   {:<15}   {:?}", 
            self.book_id, 
            self.title,
            self.author, 
            self.publication_year,
            self.issued_to.as_deref().unwrap_or("None"), // Changed {:?} to {} (5th placeholder)
            self.status, // Keep {:?} here for Enum formatting
        );

    }
}

fn save_book(books: &[Book]) -> std::io::Result<()> {
    let mut file = File::create("Book.txt")?;

    for book in books {
        writeln!(
            file, 
            "{},{},{},{},{},{:?}", // Changed 5th {:?} to {}
            book.book_id,
            book.title,
            book.author,
            book.publication_year,
            book.issued_to.as_deref().unwrap_or(""),
            book.status,
        )?;
    }

    Ok(())
}
fn load_books() -> std::io::Result<Vec<Book>> {
    let file = File::open("Book.txt")?;
    let reader = std::io::BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 6 {
            let book_id = parts[0].parse::<u32>().unwrap_or(0);
            let title = parts[1].to_string();
            let author = parts[2].to_string();
            let publication_year = parts[3].parse::<u16>().unwrap_or(0);
            let issued_to_str = parts[4].trim();
           let issued_to = match issued_to_str {
                "" => None,
                id => Some(id.to_string()),
            };
            let status = match parts[5] {
                "Available" => BookStatus::Available,
                "Issued" => BookStatus::Issued,
                _ => BookStatus::Available,
            };

            let book = Book {
                book_id,
                title,
                author,
                publication_year,
                issued_to,
                status,
            };
            books.push(book);
        }
    }

    Ok(books)
}

fn get_book_index(books:&[Book], id:u32)->Option<usize>{
    for (index, book) in books.iter().enumerate() {
        if book.book_id == id {
            return Some(index);
        }
    }
    None
}

  
fn search_book_byid<'a>(
        books:&'a Vec<Book>,
        id:u32,
    )->Result<&'a Book,String>{
 if let Some(index)=get_book_index(books,id){
    let book=&books[index];
    return Ok(book);
 }
 Err(String::from("Book Details not found"))

    }


fn delete_book(
    books:&mut Vec<Book>,
    id:u32
)->Result<(), String>{

if let Some(index) = get_book_index(books,id) {
    books.remove(index);
    return Ok(());
}
 Err(String::from("Book Details not found"))
}



fn update_book(
    books:&mut Vec<Book>,
    id:u32
)->Result<(), String>{


if let Some(index) = get_book_index(books,id) {
  let book = &mut books[index];
    book.title = add::read_string("Enter your new Title");
    book.author = add::read_string("Enter your new Author");
    book.publication_year=add::read_int("Enter your new Publication Year")as u16;
    return Ok(());

}
 Err(String::from("Book Details not found"))

}

fn issue_book(students:&Vec<Student>,books: &mut Vec<Book>, id: u32) -> Result<(), String> {
    // 1. Find the index first so we don't hold a reference to 'books'
    let index = get_book_index(books, id)
        .ok_or_else(|| String::from("Book not found"))?;

    // 2. Read the student roll number
    let stud_id = add::read_string("Enter your Roll No.");

    // 3. Verify the student exists
    search_student_rollno(students,&stud_id)?;

    // 4. Mutate the target book safely
    let book = &mut books[index];
 match book.status{
    BookStatus::Available=>{
    book.status = BookStatus::Issued;
    book.issued_to = Some(stud_id); 

    println!("Book issued successfully!");
    Ok(())
    }
     BookStatus::Issued=>{

      Err(String::from("This Book  is Not Available, Please wait until Book is Return by other student"))
     }

    }
}

fn return_book(students:&Vec<Student>,books:&mut Vec<Book>,id:u32)->Result<(),String>{
    let stud_id = add::read_string("Enter your Roll No.");
    search_student_rollno(students,&stud_id)?;

    let index = get_book_index(books, id)
        .ok_or_else(|| String::from("Book not found"))?;

let book = &mut books[index];

 match book.status{
    BookStatus::Issued=>{
              if book.issued_to.as_deref() == Some(&stud_id) {
                book.status = BookStatus::Available;
                book.issued_to = None;
                println!("Book returned successfully!");
                Ok(())
            } else {
                Err(String::from("This book was issued to a different student!"))
            }
    }

    BookStatus::Available=>{

        Err(String::from("This Book is already Available, You have already returned this Book if you had borrow earlier"))
    }
  }
  
}

fn add_book(books:&mut Vec<Book>){

    let id=add::read_int("Enter Book ID");

    if get_book_index(books, id as u32).is_some() {
    println!("Book with ID {} already exists", id);
    return;
}

    let title=add::read_string("Enter Book Title");
    let author=add::read_string("Enter Book Author");
    let year=add::read_int("Enter Publication Year");

    let book = Book::new(
    id as u32,
    title,
    author,
    year as u16,
    );
    books.push(book);
}

fn main(){
    let mut books:Vec<Book>=Vec::new();
    let mut students = Vec::<Student>::new();
       println!("Welcome to Library Management System");
        println!("");

        println!("Loading Student Details from file...");
    match load_students(){
            Ok(loaded_students)=>{
                println!("{} Student Details Loaded successfully from file",&loaded_students.len());
                students = loaded_students; // Update the main students vector with the loaded data
            }
            Err(err)=> println!("Error loading student details: {}", err),
        }
    

 
    println!("Loading Book Details from file...");
    match load_books() {
        Ok(book) =>{
            println!("{} Book Details Loaded successfully from file",&book.len());
            books=book;

        }
        Err(err) => {
            println!("Error loading book details: {}", err);
        }
    };  
    loop{
        println!("");
        println!("1. Add Book");
        println!("2. Display Books");
        println!("3. Search Book by ID");
        println!("4. Delete Book by ID");
        println!("5. Uodate Book by ID");
        println!("6. Borrow a Book");
        println!("7. Return  Book");


        println!("9. Exit");
        let choice=add::read_int("Enter your choice");
        match choice{
            1=>add_book(&mut books),
            2=>{
                println!("-------------------------------------------------------------------------------------------------------------------");
                println!("Book ID        Title                          Author               Publication Year     Issued To         Status");
                for book in &books{
                    book.display();
                }
                println!("--------------------------------------------------------------------------------------------------------------------");

            }
             3 =>{
                let id=add::read_int("Enter Book ID") as u32;

            let book=search_book_byid(&books,id);
            match book{
                Ok(book)=> {
                     println!("-------------------------------------------------------------------------------------------------------------------");
                println!("Book ID        Title                          Author               Publication Year     Issued To         Status");
                    book.display();
                println!("--------------------------------------------------------------------------------------------------------------------");

                }
                Err(err)=>println!("{err}"),
            }
             }
             
             4=>{
                let id=add::read_int("Enter Book ID")as u32;
               
           let dl= delete_book(&mut books,id);
         match dl{
           Ok(())=>  println!("Book Details Deleted successfully of Book ID: {id}"),
             
         Err(err)=>println!("{err}"),
            }
             }

             5=>{
            let id=add::read_int("Enter Book ID") as u32;

           let up= update_book(&mut books, id);
          
         match up{
           Ok(())=>  println!("Book Details Updated successfully of Book ID {id}"),
             
         Err(err)=>println!("{err}"),
            }
             }

             6=>{
            let id=add::read_int("Enter Book ID") as u32;
              let borrow=issue_book(&students,&mut books,id);
              match borrow{
                 Ok(())=>  println!("Book Issued successfully of Book ID {id}"),
             
         Err(err)=>println!("{err}"),
              }
                
             }

                7=>{
            let id=add::read_int("Enter Book ID") as u32;
              let ret=return_book(&students,&mut books,id);
              match ret{
                 Ok(())=>  println!("Book return successfully of Book ID {id}"),
             
         Err(err)=>println!("{err}"),
              }
                
             }

            9=>{
                println!("Saving Book Details to file...");
                if let Err(err) = save_book(&books) {
                    println!("Error saving book details: {}", err);
                } else {
                    println!("Book Details saved successfully to file");
                }
                println!("Exiting the program");
                break;
            }
            _=>println!("Invalid choice")
        }
    }
}
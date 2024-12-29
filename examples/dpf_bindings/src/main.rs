use dpf::{Client, Database, Error, Server};

fn main() -> Result<(), Error> {
    // Create database with 10 elements of 8 bytes each
    let mut db = Database::new(10, 8)?;
    
    // Set some elements
    db.set_element(0, &[1, 2, 3, 4, 5, 6, 7, 8])?;
    db.set_element(1, &[8, 7, 6, 5, 4, 3, 2, 1])?;
    
    // Create client and server
    let mut client = Client::new()?;
    let mut server = Server::new(&mut db)?;
    
    // Create request for index 1
    let request = client.create_request(1)?;
    
    // Process request on server
    let response = server.process_request(&request)?;
    
    // Get result on client
    let result = client.get_result(&request, &response)?;
    
    // Verify result
    assert_eq!(result, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    println!("Successfully retrieved element at index 1: {:?}", result);
    
    Ok(())
}

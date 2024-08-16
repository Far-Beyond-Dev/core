//////////////////////////////////////
// Lets import some tools we need.  //
//////////////////////////////////////

use socketioxide::extract::{SocketRef, Data};
use serde::Deserialize;
use tracing::info;
use crate::{define_event, players};
 
/// Initializes the chat subsystem.
///
/// This function sets up event listeners for various chat-related events
/// such as whisper, broadcast, and command (help).
///
/// # Arguments
///
/// * `socket` - A reference to the socket connection for the player.
pub fn init(socket: SocketRef) {
    info!("Starting chat subsystem...");
   
    let def_socket: SocketRef = socket.clone();
    define_event!(def_socket,
        "whisper", handle_whisper(socket.clone(), "", "", ""),
        "broadcast", handle_broadcast(socket.clone(), "", ""),
        "command", handle_help(socket.clone(), "")
    );
}
   
    ////////////////////////////////////////////////////
    //  Finally we define the functions that will be  //
    //  called by our event listeners when they are   //
    //  triggered.                                    //
    ////////////////////////////////////////////////////
   
    /// Handles whisper messages between players.
    ///
    /// This function processes a whisper message from one player to another
    /// and emits the message to the recipient.
    ///
    /// # Arguments
    ///
    /// * `socket` - A reference to the socket connection.
    /// * `sender` - The name or identifier of the message sender.
    /// * `recipient` - The name or identifier of the message recipient.
    /// * `message` - The content of the whisper message.
    fn handle_whisper(socket: SocketRef, sender: &str, recipient: &str, message: &str) {
        info!("Whisper from {} to {}: {}", sender, recipient, message);
        // Example of emitting a message to a specific recipient
        socket.emit("whisper", (recipient, message)).ok();
    }
   
    /// Handles help requests from players.
    ///
    /// This function processes a help request from a player and sends
    /// a help message back to the requester.
    ///
    /// # Arguments
    ///
    /// * `socket` - A reference to the socket connection.
    /// * `sender` - The name or identifier of the player requesting help.
    fn handle_help(socket: SocketRef, sender: &str) {
        info!("Help requested by {}", sender);
        // Example of emitting a help message to the sender
        socket.emit("help", "Here's some help!").ok();
    }
   
    /// Handles broadcast messages to all players.
    ///
    /// This function processes a broadcast message from a player and
    /// sends it to all connected clients.
    ///
    /// # Arguments
    ///
    /// * `socket` - A reference to the socket connection.
    /// * `sender` - The name or identifier of the message sender.
    /// * `message` - The content of the broadcast message.
    fn handle_broadcast(socket: SocketRef, sender: &str, message: &str) {
        info!("Broadcast from {}: {}", sender, message);
        // Example of broadcasting a message to all connected clients
        socket.broadcast().emit("broadcast", message).ok();
    }
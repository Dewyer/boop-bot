pub enum IrcMessage
{
	Message(IrcPrivateMessage),
	Other(String),
	None
}

pub struct IrcPrivateMessage
{
	pub content:String,
	pub author_nick:String,
	pub author_full:String,
	pub target:String
}

impl IrcMessage
{
	pub fn from(message:&str) -> Self
	{
		let tokens:Vec<&str> = message.split(" ").collect::<Vec<&str>>();
		if tokens.len() >= 3
		{
			let sender = tokens[0];
			let cmd = tokens[1];
			let chn = tokens[2];
			if cmd == "PRIVMSG"
			{
				let mut cnt:String = message.split(&format!("{} {} {}",sender,cmd,chn)).collect::<Vec<&str>>()[1][2..].to_string();
				if cnt.ends_with("\n")
				{
					cnt.pop();
				}
				if cnt.ends_with("\r")
				{
					cnt.pop();
				}

				let author_parts = sender[1..].split("!").collect::<Vec<&str>>();
				return IrcMessage::Message(IrcPrivateMessage{
					content:cnt,
					author_full:String::from(author_parts[1]),
					author_nick:String::from(author_parts[0]),
					target:String::from(chn)
				});
			}
		}

		IrcMessage::Other(String::from(message))

	}
}

#[derive(Debug)]
pub struct Command
{
	prefix:String,
	command:String,
	arguments:Vec<String>
}

impl Command
{
	pub fn from(msg:&str,prefix:&str) -> Option<Command>
	{
		if msg.starts_with(prefix)
		{
			let mut cmd = String::new();
			let mut last_part = String::new();
			let mut params:Vec<String> = Vec::new();
			for (ii,chr) in msg.chars().enumerate()
			{
				if ii != 0
				{
					if chr == ' ' && cmd.is_empty()
					{
						cmd = last_part.clone();
					}
					if chr == '{'
					{
						if cmd.is_empty()
						{
							cmd = last_part.clone();
						}
						last_part = String::new();
					}
					else if chr == '}'
					{
						params.push(last_part.clone());
					}

					if chr != '{'
					{
						last_part.push(chr);
					}
				}
			}

			if cmd.is_empty()
			{
				cmd = last_part.clone();
			}

			Some(Command{
				prefix:prefix.to_string(),
				command:cmd,
				arguments:params
			})
		}
		else{
			None
		}
	}
}
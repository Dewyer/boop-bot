use std::net::TcpStream;
use anyhow::Result;
use std::io::{Write, BufReader};
use std::iter;
use std::time::Duration;
use std::io::BufRead;

use crate::ircclient::message::IrcMessage;
pub mod message;

const DEFAULT_IRC_PORT:isize = 6667;

pub struct IrcClient
{
	config:Config,
	socket:Option<TcpStream>
}

pub struct IrcCommand
{
	command:String,
	params:Vec<String>
}

#[derive(Debug,Clone)]
pub struct Config
{
	pub server:String,
	pub nickname:String,
	pub port: Option<isize>
}


impl IrcCommand
{
	pub fn new(cmd:&str,params:Vec<String>) -> Self
	{
		return IrcCommand
		{
			command:String::from(cmd),
			params
		}
	}
}

macro_rules! get_soc {
    ($self:ident) => ($self.socket.as_ref().ok_or(anyhow::Error::msg("wtf")));
}

impl IrcClient
{
	pub fn new(config:Config) -> Self
	{
		return IrcClient
		{
			config,
			socket:None
		}
	}

	pub fn read_line(&mut self) -> Result<String>
	{
		let soc = get_soc!(self)?;
		let mut resp_str = String::new();
		let mut buff = BufReader::new(soc);
		buff.read_line(&mut resp_str)?;
		Ok(resp_str)
	}

	pub fn send_message(&mut self,target:&str,msg:&str) -> Result<()>
	{
		let priv_msg = IrcCommand::new("PRIVMSG",vec![String::from(target),String::from(msg)]);
		self.send_command_without_response(&priv_msg)?;
		Ok(())
	}

	pub fn send_ping(&mut self) -> Result<()>
	{
		let ping = IrcCommand::new("PONG",vec![String::from(" :pingisn")]);
		self.send_command_without_response(&ping)?;
		Ok(())
	}

	pub fn send_command(&mut self,cmd:&IrcCommand) -> Result<String>
	{
		self.send_command_without_response(cmd)?;
		let resp_str = self.read_line()?;
		Ok(resp_str)
	}

	pub fn send_command_without_response(&mut self, cmd:&IrcCommand ) -> Result<()>
	{
		let mut soc = get_soc!(self)?;
		let str_msg = format!("{} {}\n",cmd.command,cmd.params.join(" "));
		soc.write(str_msg.as_bytes())?;
		println!("Sent command {}",cmd.command);
		Ok(())
	}

	pub fn connect(&mut self) -> Result<()>
	{
		self.socket = Some(TcpStream::connect(format!("{}:{}",self.config.server,self.config.port.or(Some(DEFAULT_IRC_PORT)).unwrap()))?);
		self.socket.as_ref().unwrap().set_read_timeout(Some(Duration::new(5,0)))?;

		let user_cmd = IrcCommand::new("USER",Self::get_nick_params(self.config.nickname.clone().as_str()));
		self.send_command_without_response(&user_cmd)?;
		let nick_cmd = IrcCommand::new("NICK",vec![self.config.nickname.clone()]);
		self.send_command_without_response(&nick_cmd)?;

		Ok(())
	}

	pub fn join_channel(&mut self,channel:&str) -> Result<()>
	{
		let join_cmd = IrcCommand::new("JOIN",vec![String::from(channel)]);
		self.send_command_without_response(&join_cmd)
	}

	fn get_nick_params(nick:&str) -> Vec<String>
	{
		let mut user_params : Vec<String> = iter::repeat(String::from(nick)).take(4).collect();
		user_params
	}


	pub fn read_next_message(&mut self) -> Result<IrcMessage>
	{
		let mut soc = get_soc!(self)?;
		let mut test_buf = [0;10];
		let peek_res = soc.peek(&mut test_buf);

		if peek_res.is_ok()
		{
			let mut got_str = self.read_line()?;

			Ok(IrcMessage::from(&got_str))
		}
		else
		{
			Ok(IrcMessage::None)
		}
	}
}

pub struct Server{
    hostname: &'static str,
    port: u16
}

impl Server {
    
    pub fn new(hostname: &'static str, port: u16) -> Server {
        Server { hostname:hostname, port:port }
    }
    
    pub fn hostname(&self) -> &'static str {
        self.hostname
    }    

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Copy for Server { }

pub struct User {
    nick: &'static str,
    ident: &'static str,
    hostname: &'static str,
    realname: &'static str
}

impl User {
    
    pub fn new(nick: &'static str, ident: &'static str, hostname: &'static str, realname: &'static str) -> User {
        User { nick:nick, ident:ident, hostname:hostname, realname:realname }
    }
    
    pub fn nick(&self) -> &'static str {
        self.nick
    }
    
    pub fn ident(&self) -> &'static str {
        self.ident
    }
    
    pub fn host(&self) -> &'static str {
        self.hostname
    }
    
    pub fn realname(&self) -> &'static str {
        self.realname
    }    
}

impl Copy for User { }

pub struct Channel {
    name: &'static str,
    usercount: u16
}

impl Channel {
    pub fn new(name: &'static str) -> Channel {
        Channel { name:name, usercount: 0u16 }
    }
    
    pub fn usercount(&self) -> u16 {
        self.usercount
    }
    
    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Copy for Channel { }

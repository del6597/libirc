pub struct Server{
    hostname: &'static str,
    port: u16
}

impl Server {
    pub fn hostname(&self) -> &'static str {
        self.hostname
    }    

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub struct User {
    nick: &'static str,
    ident: &'static str,
    hostname: &'static str,
    realname: &'static str
}

impl User {
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

pub struct Channel {
    name: &'static str,
    usercount: u16
}

impl Channel {
    pub fn usercount(&self) -> u16 {
        self.usercount
    }
    
    pub fn name(&self) -> &'static str {
        self.name
    }
}

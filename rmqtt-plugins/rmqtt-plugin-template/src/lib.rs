use async_trait::async_trait;
use rmqtt::{
    broker::hook::{self, Handler, Parameter, Register, Results, ReturnType},
    plugin::Plugin,
    Result, Runtime,
};

#[inline]
pub async fn init<N: Into<String>, D: Into<String>>(
    runtime: &'static Runtime,
    name: N,
    descr: D,
    default_startup: bool,
) -> Result<()> {
    runtime
        .plugins
        .register(
            Box::new(Template::new(runtime, name.into(), descr.into()).await),
            default_startup,
        )
        .await?;
    Ok(())
}

struct Template {
    _runtime: &'static Runtime,
    name: String,
    descr: String,
    register: Box<dyn Register>,
}

impl Template {
    #[inline]
    async fn new(runtime: &'static Runtime, name: String, descr: String) -> Self {
        let register = runtime.extends.hook_mgr().await.register();
        Self {
            _runtime: runtime,
            name,
            descr,
            register,
        }
    }
}

#[async_trait]
impl Plugin for Template {
    #[inline]
    async fn init(&mut self) -> Result<()> {
        log::info!("{} init", self.name);

        self.register
            .add(hook::Type::BeforeStartup, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::SessionCreated, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::ClientConnect, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::ClientConnack, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::ClientConnected, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::ClientDisconnected, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::ClientSubscribe, Box::new(HookHandler {}));
        self.register
            .add(hook::Type::MessagePublish, Box::new(HookHandler {}));
        Ok(())
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    async fn start(&mut self) -> Result<()> {
        log::info!("{} start", self.name);
        self.register.resume();
        Ok(())
    }

    #[inline]
    async fn stop(&mut self) -> Result<bool> {
        log::info!("{} stop", self.name);
        self.register.suspend();
        Ok(true)
    }

    #[inline]
    fn version(&self) -> &str {
        "0.1.1"
    }

    #[inline]
    fn descr(&self) -> &str {
        &self.descr
    }
}

struct HookHandler {}

#[async_trait]
impl Handler for HookHandler {
    async fn hook(&mut self, param: &Parameter, results: Results) -> ReturnType {
        match param {
            Parameter::BeforeStartup => {
                log::info!("before startup");
            }
            Parameter::SessionCreated(_session, c) => {
                log::info!("{:?} session created", c.id);
            }
            Parameter::ClientConnect(_session, c) => {
                log::info!("{:?} client connect", c.id);
            }
            Parameter::ClientConnack(_session, c, r) => {
                log::info!("{:?} client connack, {:?}", c.id, r);
            }
            Parameter::ClientConnected(_session, c) => {
                log::info!("{:?} client connected", c.id);
            }
            Parameter::ClientDisconnected(_session, c, reason) => {
                log::info!("{:?} client disconnected, reason: {}", c.id, reason);
            }
            Parameter::ClientSubscribe(_session, c, subscribe) => {
                log::info!("{:?} client subscribe, {:?}", c.id, subscribe);
            }
            Parameter::MessagePublish(_session, c, publish) => {
                log::info!("{:?} message publish, {:?}", c.id, publish);
            }
            _ => {
                log::error!("unimplemented, {:?}", param)
            }
        }
        (true, results)
    }
}

use crate::handler::Handler;

pub enum Route {
    ParamNode {
        children: Vec<Route>,
        handler: Option<Handler>,
        name: String,
    },
    PathNode {
        children: Vec<Route>,
        handler: Option<Handler>,
        name: String,
    },
    RootNode {
        children: Vec<Route>,
        handler: Option<Handler>,
    },
}

impl Route {
    pub fn parse(path: &str, handler: Handler) -> Route {
        let mut root = Route::default();

        if path == "/" {
            root.set_handler(handler);
        } else {
        }

        root
    }

    pub fn handler(&self) -> &Option<Handler> {
        match self {
            Self::ParamNode {
                children: _,
                handler: h,
                name: _,
            } => h,
            Self::PathNode {
                children: _,
                handler: h,
                name: _,
            } => h,
            Self::RootNode {
                children: _,
                handler: h,
            } => h,
        }
    }

    pub fn search(&self, path: &str) -> Option<Handler> {
        None
    }

    pub fn set_handler(&mut self, handler: Handler) {
        match self {
            Self::ParamNode {
                children: _,
                handler: h,
                name: _,
            } => {
                *h = Some(handler);
            }
            Self::PathNode {
                children: _,
                handler: h,
                name: _,
            } => {
                *h = Some(handler);
            }
            Self::RootNode {
                children: _,
                handler: h,
            } => {
                *h = Some(handler);
            }
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::RootNode {
            children: Vec::new(),
            handler: None,
        }
    }
}

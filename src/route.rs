use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::handler::Handler;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RouteNodeType {
    ParamNode(String),
    PathNode(String),
    RootNode,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RouteNode {
    children: Vec<Box<RouteNode>>,
    handler: Option<Handler>,
    node_type: RouteNodeType
}

pub struct Router {
    root: RouteNode,
}

impl Router {
    fn parse(path: &str) -> Vec<RouteNodeType> {
        let mut parts = vec![RouteNodeType::RootNode];

        if path == "/" {
            return parts;
        }

        for (index, path_part) in path.split('/').enumerate() {
            if index == 0 {
                continue;
            }

            parts.push(if path_part.starts_with(':') {
                RouteNodeType::ParamNode(path_part[1..].to_string())
            } else {
                RouteNodeType::PathNode(path_part.to_string())
            });
        }

        parts
    }

    pub fn add_handler(&mut self, path: &str, handler: Handler) {
        let route_parts = Router::parse(path);
        let mut depth = 0;

        let mut cursor = Rc::new(RefCell::new(&mut self.root));

        while depth < route_parts.len() - 1 {
            let mut is_matched = false;

            for child in cursor.borrow_mut().children.iter_mut() {
                if child.node_type == route_parts[depth] {
                    cursor = Rc::new(RefCell::new(child));
                    is_matched = true;
                    break;
                }
            }

            if !is_matched {
                let mut new_node = RouteNode {
                    children: Vec::new(),
                    handler: None,
                    node_type: route_parts[depth].clone(),
                };

                cursor.borrow_mut().children.push(new_node);
                cursor = Rc::new(RefCell::new(cursor.borrow_mut().children.last_mut().unwrap()));
            }
        }

        cursor.borrow_mut().handler = Some(handler);
    }

    pub fn new() -> Self {
        Self {
            root: RouteNode {
                children: Vec::new(),
                handler: None,
                node_type: RouteNodeType::RootNode,
            }
        }
    }

    pub fn search(&self, path: &str) -> Option<Handler> {
        if path == "/" {
            return self.root.handler;
        }

        let path_parts = path.split('/').collect::<Vec<&str>>();

        fn recursive_search(path_parts: &Vec<&str>, node: &RouteNode, path_part_index: usize) -> Vec<Handler> {
            let mut matched_handlers = Vec::new();

            let is_matched = match &node.node_type {
                RouteNodeType::ParamNode(_) => true,
                RouteNodeType::PathNode(n) => Some(n.as_str()) == path_parts.get(path_part_index).map(|p| *p),
                _ => false,
            };

            if is_matched {
                if path_part_index == path_parts.len() - 1 {
                    if let Some(matched_handler) = node.handler {
                        matched_handlers.push(matched_handler);
                    }
                } else {
                    for child in &node.children {
                        matched_handlers.append(&mut recursive_search(path_parts, child, path_part_index + 1));
                    }
                }
            }

            matched_handlers
        }

        let mut matched_handlers = Vec::new();

        for child in &self.children {
            let mut child_matched_handlers = recursive_search(&path_parts, &child, 1);

            matched_handlers.append(&mut child_matched_handlers);
        }

        matched_handlers.get(0).copied()
    }
}

impl Default for RouteNode {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            handler: None,
            node_type: RouteNodeType::RootNode,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{context::Context, Request, Response};

    use super::*;

    fn noop_handler(_: Request, _: Context) -> Response {
        Response::with_text("OK")
    }

    #[test]
    fn test_parse_root_route() {
        let route_node = RouteNode::parse("/", noop_handler);

        assert_eq!(route_node, RouteNode {
            handler: Some(noop_handler),
            ..Default::default()
        });
    }

    #[test]
    fn test_parse_simple_route() {
        let route_node = RouteNode::parse("/foo/bar", noop_handler);

        assert_eq!(route_node, RouteNode {
            children: vec![
                RouteNode {
                    children: vec![
                        RouteNode {
                            handler: Some(noop_handler),
                            node_type: RouteNodeType::PathNode("bar".to_string()),
                            ..Default::default()
                        },
                    ],
                    node_type: RouteNodeType::PathNode("foo".to_string()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        });
    }

    #[test]
    fn test_parse_route_with_param() {
        let route_node = RouteNode::parse("/foo/:bar/buzz", noop_handler);

        assert_eq!(route_node, RouteNode {
            children: vec![
                RouteNode {
                    children: vec![
                        RouteNode {
                            children: vec![
                                RouteNode {
                                    handler: Some(noop_handler),
                                    node_type: RouteNodeType::PathNode("buzz".to_string()),
                                    ..Default::default()
                                },
                            ],
                            node_type: RouteNodeType::ParamNode("bar".to_string()),
                            ..Default::default()
                        },
                    ],
                    node_type: RouteNodeType::PathNode("foo".to_string()),
                    ..Default::default()
                }
            ],
            ..Default::default()
        });
    }

    #[test]
    fn test_search_root_route() {
        let route_node = RouteNode::parse("/", noop_handler);

        let result = route_node.search("/");

        assert_eq!(result, Some(noop_handler as fn(Request, Context) -> Response));
    }

    #[test]
    fn test_search_simple_route() {
        let route_node = RouteNode::parse("/foo/bar", noop_handler);

        let result = route_node.search("/foo/bar");

        assert_eq!(result, Some(noop_handler as fn(Request, Context) -> Response));
    }

    #[test]
    fn test_search_route_with_param() {
        let route_node = RouteNode::parse("/foo/:bar/buzz", noop_handler);

        let result = route_node.search("/foo/123/buzz");

        assert_eq!(result, Some(noop_handler as fn(Request, Context) -> Response));
    }

    #[test]
    fn test_search_not_found() {
        let route_node = RouteNode::parse("/foo/bar", noop_handler);

        let result = route_node.search("/foo");

        assert_eq!(result, None);
    }
}
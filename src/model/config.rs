use crate::model::config::behavior::Behavior;
use crate::model::config::color::Color;
use crate::model::config::composition::Composition;
use crate::model::config::debug::Debug;
use crate::model::config::setup::Setup;
use crate::utils::print_help;
use crate::utils::read_file;
use serde::Deserialize;
use std::env::args;
use std::process::exit;
use toml;

mod args_parser;
mod behavior;
mod color;
mod composition;
mod debug;
mod setup;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub behavior: Behavior,

    #[serde(default)]
    pub color: Color,

    #[serde(default)]
    pub composition: Composition,

    #[serde(default)]
    pub debug: Debug,

    #[serde(default)]
    pub setup: Setup,
}

impl Config {
    pub fn new() -> Self {
        let config = Self::read_config_file_from_home().unwrap_or_else(Self::default);

        Self::parse_args(config, args().skip(1))
    }

    fn split_arg(arg: String) -> (String, String) {
        let split_arg: Vec<&str> = arg.split('=').collect();

        if split_arg.len() == 1 {
            return (String::from(split_arg[0]), String::from(""));
        }

        (String::from(split_arg[0]), String::from(split_arg[1]))
    }

    fn parse_value<F>((key, value): (String, String)) -> F
    where
        F: std::str::FromStr,
    {
        value.parse().unwrap_or_else(|_| {
            println!("option '{}={}' was not parsable", key, value);
            exit(1);
        })
    }

    fn read_config_file_from_home() -> Option<Self> {
        if let Ok(home_dir) = std::env::var("HOME") {
            let home_config_path = format!("{}/{}", home_dir, ".twilight-commander-rc.toml");
            if let Ok(config_file) = read_file(&home_config_path) {
                return toml::from_str(&config_file).ok();
            }
        }

        None
    }

    fn parse_args<T>(mut config: Self, args: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        for arg in args {
            let (key, value) = Self::split_arg(arg);

            match key.as_str() {
                "--behavior.file_action" => config.behavior.file_action = Self::parse_value((key, value)),
                "--behavior.path_node_sort" => config.behavior.path_node_sort = Self::parse_value((key, value)),
                "--behavior.scrolling" => config.behavior.scrolling = Self::parse_value((key, value)),
                "--color.background" => config.color.background = Self::parse_value((key, value)),
                "--color.foreground" => config.color.foreground = Self::parse_value((key, value)),
                "--composition.indent" => config.composition.indent = Self::parse_value((key, value)),
                "--composition.show_indent" => config.composition.show_indent = Self::parse_value((key, value)),
                "--composition.use_utf8" => config.composition.use_utf8 = Self::parse_value((key, value)),
                "--debug.enabled" => config.debug.enabled = Self::parse_value((key, value)),
                "--debug.padding_bot" => config.debug.padding_bot = Self::parse_value((key, value)),
                "--debug.padding_top" => config.debug.padding_top = Self::parse_value((key, value)),
                "--debug.spacing_bot" => config.debug.spacing_bot = Self::parse_value((key, value)),
                "--debug.spacing_top" => config.debug.spacing_top = Self::parse_value((key, value)),
                "--setup.working_dir" => config.setup.working_dir = Self::parse_value((key, value)),

                "--help" => print_help(),
                "--" => break,
                _ => {
                    println!("unknown option {}", key);
                    exit(1);
                }
            }
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let default_config = Config::default();
        let args_vec = vec![
            String::from("--behavior.file_action=file_action_test"),
            String::from("--behavior.path_node_sort=path_node_sort_test"),
            String::from("--behavior.scrolling=scrolling_test"),
            String::from("--color.background=background_test"),
            String::from("--color.foreground=foreground_test"),
            String::from("--debug.enabled=true"),
            String::from("--debug.padding_bot=111"),
            String::from("--debug.padding_top=222"),
            String::from("--debug.spacing_bot=333"),
            String::from("--debug.spacing_top=444"),
            String::from("--setup.working_dir=working_dir_test"),
        ];

        let config = Config::parse_args(default_config, args_vec.into_iter());

        assert_eq!(config.behavior.file_action, String::from("file_action_test"));
        assert_eq!(config.behavior.path_node_sort, String::from("path_node_sort_test"));
        assert_eq!(config.behavior.scrolling, String::from("scrolling_test"));
        assert_eq!(config.color.background, String::from("background_test"));
        assert_eq!(config.color.foreground, String::from("foreground_test"));
        assert_eq!(config.debug.enabled, true);
        assert_eq!(config.debug.padding_bot, 111);
        assert_eq!(config.debug.padding_top, 222);
        assert_eq!(config.debug.spacing_bot, 333);
        assert_eq!(config.debug.spacing_top, 444);
        assert_eq!(config.setup.working_dir, String::from("working_dir_test"));
    }

    #[test]
    fn test_parse_args_with_stopper() {
        let default_config = Config::default();
        let args_vec = vec![
            String::from("--behavior.file_action=file_action_test"),
            String::from("--behavior.path_node_sort=path_node_sort_test"),
            String::from("--behavior.scrolling=scrolling_test"),
            String::from("--color.background=background_test"),
            String::from("--color.foreground=foreground_test"),
            String::from("--"),
            String::from("--debug.enabled=true"),
            String::from("--debug.padding_bot=111"),
            String::from("--debug.padding_top=222"),
            String::from("--debug.spacing_bot=333"),
            String::from("--debug.spacing_top=444"),
            String::from("--setup.working_dir=working_dir_test"),
        ];

        let config = Config::parse_args(default_config, args_vec.into_iter());
        let def_conf = Config::default();

        assert_eq!(config.behavior.file_action, String::from("file_action_test"));
        assert_eq!(config.behavior.path_node_sort, String::from("path_node_sort_test"));
        assert_eq!(config.behavior.scrolling, String::from("scrolling_test"));
        assert_eq!(config.color.background, String::from("background_test"));
        assert_eq!(config.color.foreground, String::from("foreground_test"));
        assert_eq!(config.debug.enabled, def_conf.debug.enabled);
        assert_eq!(config.debug.padding_bot, def_conf.debug.padding_bot);
        assert_eq!(config.debug.padding_top, def_conf.debug.padding_top);
        assert_eq!(config.debug.spacing_bot, def_conf.debug.spacing_bot);
        assert_eq!(config.debug.spacing_top, def_conf.debug.spacing_top);
        assert_eq!(config.setup.working_dir, def_conf.setup.working_dir);
    }
}

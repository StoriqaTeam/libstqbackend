use regex::Regex;

pub type ParamsConverter<T> = Box<Fn(Vec<&str>) -> Option<T>>;

/// `RouteParser` class maps regex to type-safe list of routes, defined by `enum Route`
pub struct RouteParser<T> {
    regex_and_converters: Vec<(Regex, ParamsConverter<T>)>,
}

impl<T> Default for RouteParser<T> {
    fn default() -> Self {
        Self {
            regex_and_converters: Default::default(),
        }
    }
}

impl<T> RouteParser<T> {
    /// Adds mapping between regex and route
    /// #Examples
    ///
    /// ```
    /// use stq_http::router::RouteParser;
    ///
    /// #[derive(Debug)]
    /// pub enum Route {
    ///     Users,
    /// }
    ///
    /// let mut router = RouteParser::default();
    /// router.add_route(r"^/users$", || Route::Users);
    /// ```
    pub fn add_route<F>(&mut self, regex_pattern: &str, f: F) -> &Self
    where
        F: Fn() -> T + 'static,
    {
        self.add_route_with_params(regex_pattern, move |_| Some(f()));
        self
    }

    /// Adds mapping between regex and route with params
    /// converter is a function with argument being a set of regex matches (strings) for route params in regex
    /// this is needed if you want to convert params from strings to int or some other types
    ///
    /// #Examples
    ///
    /// ```
    /// use stq_http::router::RouteParser;
    ///
    /// #[derive(Debug)]
    /// pub enum Route {
    ///     User(i32),
    /// }
    ///
    /// let mut router = RouteParser::default();
    /// router.add_route_with_params(r"^/users/(\d+)$", |params| {
    ///     params.get(0)
    ///        .and_then(|string_id| string_id.parse::<i32>().ok())
    ///        .map(|user_id| Route::User(user_id))
    /// });
    /// ```
    pub fn add_route_with_params<F>(&mut self, regex_pattern: &str, converter: F) -> &Self
    where
        F: Fn(Vec<&str>) -> Option<T> + 'static,
    {
        let regex = Regex::new(regex_pattern).unwrap();
        self.regex_and_converters.push((regex, Box::new(converter)));
        self
    }

    /// Tests string router for matches
    /// Returns Some(route) if there's a match
    /// #Examples
    ///
    /// ```
    /// use stq_http::router::RouteParser;
    ///
    /// #[derive(Debug, PartialEq)]
    /// pub enum Route {
    ///     Users,
    /// }
    ///
    /// let mut router = RouteParser::default();
    /// router.add_route(r"^/users$", || Route::Users);
    /// let route = router.test("/users").unwrap();
    /// assert_eq!(route, Route::Users);
    /// ```
    pub fn test(&self, route: &str) -> Option<T> {
        self.regex_and_converters
            .iter()
            .fold(None, |acc, regex_and_converter| {
                if acc.is_some() {
                    return acc;
                }
                RouteParser::<T>::get_matches(&regex_and_converter.0, route).and_then(|v| regex_and_converter.1(v))
            })
    }

    fn get_matches<'a>(regex: &Regex, string: &'a str) -> Option<Vec<&'a str>> {
        regex.captures(string).and_then(|captures| {
            captures
                .iter()
                .skip(1)
                .fold(Some(Vec::<&str>::new()), |mut maybe_acc, maybe_match| {
                    if let Some(ref mut acc) = maybe_acc {
                        if let Some(mtch) = maybe_match {
                            acc.push(mtch.as_str());
                        }
                    }
                    maybe_acc
                })
        })
    }
}

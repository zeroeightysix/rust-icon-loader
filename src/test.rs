#[cfg(test)]
mod test {
    use crate::{IconFileType, IconLoader};

    #[test]
    fn test_find_firefox_icon() {
        let loader = IconLoader::new_hicolor();

        let icon = loader.load_icon("firefox").unwrap();
        let icon = icon.file_for_size_scaled(32, 1);

        assert_eq!(icon.dir_info().path().to_str(), Some("32x32/apps"));
        assert_eq!(icon.dir_info().scale(), 1);
        assert_eq!(icon.icon_type(), IconFileType::PNG);
    }
}

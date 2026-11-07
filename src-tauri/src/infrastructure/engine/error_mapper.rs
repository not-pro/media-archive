use crate::domain::error::AppError;

fn rule(
    output: &str,
    patterns: &[&str],
    code: &'static str,
    message: &'static str,
) -> Option<AppError> {
    patterns
        .iter()
        .any(|p| output.contains(p))
        .then(|| AppError::new(code, message))
}

pub fn map_engine_error(output: &str) -> AppError {
    let text = output.to_lowercase();
    let rules: &[(&[&str], &str, &str)] = &[
        (
            &["drm", "widevine"],
            "DRM_UNSUPPORTED",
            "Материал защищён DRM. Скачивание защищённых потоков не поддерживается.",
        ),
        (
            &["po token", "po_token", "missing_pot"],
            "PO_TOKEN_REQUIRED",
            "Сервис запросил PO Token. Попробуйте другое качество или обновите приложение.",
        ),
        (
            &["members-only", "members only", "membership"],
            "MEMBERS_ONLY",
            "Материал доступен только участникам канала. Проверьте подписку и cookies.",
        ),
        (
            &["private video", "granted access", "no permission"],
            "ACCESS_DENIED",
            "У текущей сессии нет доступа к этому материалу.",
        ),
        (
            &["age-restrict", "confirm your age", "age verification"],
            "AGE_RESTRICTED",
            "Сервис требует подтверждение возраста в аккаунте.",
        ),
        (
            &[
                "sign in",
                "login required",
                "cookies expired",
                "cookies invalid",
            ],
            "AUTH_REQUIRED",
            "Нужен вход или сессия истекла. Импортируйте свежие cookies.",
        ),
        (
            &[
                "too many requests",
                "http error 429",
                "rate limit",
                "try again later",
            ],
            "RATE_LIMITED",
            "Сервис временно ограничил запросы. Подождите и повторите попытку.",
        ),
        (
            &["no space", "disk full", "not enough space"],
            "DISK_FULL",
            "Недостаточно свободного места на диске.",
        ),
        (
            &["file exists", "already been downloaded"],
            "FILE_EXISTS",
            "Такой файл уже существует. Разрешите замену или выберите другую папку.",
        ),
        (
            &[
                "timed out",
                "timeout",
                "connection",
                "network",
                "temporary failure",
            ],
            "NETWORK_ERROR",
            "Ошибка сети. Проверьте подключение и повторите попытку.",
        ),
        (
            &["no video formats", "requested format", "unsupported url"],
            "FORMAT_UNAVAILABLE",
            "Нужный формат недоступен. Попробуйте другое качество.",
        ),
    ];
    for (patterns, code, message) in rules {
        if let Some(error) = rule(&text, patterns, code, message) {
            return error;
        }
    }
    let detail = output
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .unwrap_or("неизвестная ошибка");
    AppError::new(
        "DOWNLOAD_FAILED",
        format!("Загрузчик завершился с ошибкой: {detail}"),
    )
}

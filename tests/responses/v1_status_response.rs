use chrono::{FixedOffset, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::{Localized, Status, StatusUpdate, V1StatusData, ValorantApiError, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-status/bad_request.json");

    let expected: ValorantApiResponse<V1StatusData> = ValorantApiResponse {
        status: 400,
        name: None,
        tag: None,
        errors: Some(
            vec![
                ValorantApiError {
                    code: 0,
                    message: String::from("string"),
                    details: String::from("string"),
                }
            ]
        ),
        results: None,
        data: None,
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_empty() {
    let json = read_resource("v1-status/ok_empty.json");

    let expected = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V1StatusData {
                maintenances: vec![],
                incidents: vec![],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_example() {
    let json = read_resource("v1-status/ok_example.json");

    let expected = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V1StatusData {
                maintenances: vec![
                    Status {
                        id: 4175,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                            .unwrap()
                            .with_nanosecond(530_569_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        archived: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 13, 10, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        incident_severity: Some(String::from("warning")),
                        maintenance_status: Some(String::from("in_progress")),
                        titles: vec![
                            Localized {
                                content: String::from("Emergency Maintenance In Progress"),
                                locale: String::from("en_US"),
                            }
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 6658,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                                    .unwrap()
                                    .with_nanosecond(645_499_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("riotclient"),
                                ],
                                translations: vec![
                                    Localized {
                                        content: String::from("The platform is currently unavailable while we perform emergency maintenance."),
                                        locale: String::from("en_US"),
                                    }
                                ],
                            }
                        ],
                    }
                ],
                incidents: vec![
                    Status {
                        id: 4175,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                            .unwrap()
                            .with_nanosecond(530_569_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        archived: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 13, 10, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        incident_severity: Some(String::from("warning")),
                        maintenance_status: Some(String::from("in_progress")),
                        titles: vec![
                            Localized {
                                content: String::from("Emergency Maintenance In Progress"),
                                locale: String::from("en_US"),
                            }
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 6658,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                                    .unwrap()
                                    .with_nanosecond(645_499_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("riotclient"),
                                ],
                                translations: vec![
                                    Localized {
                                        content: String::from("The platform is currently unavailable while we perform emergency maintenance."),
                                        locale: String::from("en_US"),
                                    }
                                ],
                            }
                        ],
                    }
                ],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_issue() {
    let json = read_resource("v1-status/ok_issue.json");

    let expected = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V1StatusData {
                maintenances: vec![],
                incidents: vec![
                    Status {
                        id: 8082,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2024, 2, 21, 16, 21, 56)
                            .unwrap()
                            .with_nanosecond(676_828_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2024, 2, 26, 0, 27, 9)
                                .unwrap()
                                .with_nanosecond(462_588_000)
                                .unwrap()
                        ),
                        archived: None,
                        incident_severity: Some(String::from("info")),
                        maintenance_status: None,
                        titles: vec![
                            Localized {
                                locale: String::from("en_US"),
                                content: String::from("Payment Issues"),
                            },
                            Localized {
                                locale: String::from("ru_RU"),
                                content: String::from("Проблемы с оплатой\r\n"),
                            },
                        ],
                        platforms: vec![
                            String::from("windows"),
                            String::from("macos"),
                            String::from("android"),
                            String::from("ios"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 14057,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2024, 2, 21, 16, 21, 56)
                                    .unwrap()
                                    .with_nanosecond(683_227_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2024, 2, 21, 16, 21, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("game"),
                                    String::from("riotclient"),
                                    String::from("riotstatus"),
                                ],
                                translations: vec![
                                    Localized {
                                        locale: String::from("en_US"),
                                        content: String::from("We're aware of a problem that's causing VP purchases to fail and are working on a fix."),
                                    },
                                    Localized {
                                        locale: String::from("ru_RU"),
                                        content: String::from("Мы знаем о проблеме, из-за которой не удается приобрести VP, и работаем над ее решением."),
                                    },
                                ],
                            }
                        ],
                    },
                    Status {
                        id: 8148,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2024, 3, 1, 22, 37, 43)
                            .unwrap()
                            .with_nanosecond(103_270_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2024, 3, 3, 23, 49, 48)
                                .unwrap()
                                .with_nanosecond(652_038_000)
                                .unwrap(),
                        ),
                        archived: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2024, 3, 6, 2, 30, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        incident_severity: Some(String::from("warning")),
                        maintenance_status: None,
                        titles: vec![
                            Localized {
                                locale: String::from("en_US"),
                                content: String::from("Competitive Queue is Ending"),
                            },
                            Localized {
                                locale: String::from("ar_AE"),
                                content: String::from("يوشك طابور اللعب التنافسي على الانتهاء"),
                            },
                            Localized {
                                locale: String::from("de_DE"),
                                content: String::from("Gewertete Warteschlange endet"),
                            },
                            Localized {
                                locale: String::from("en_SG"),
                                content: String::from("Competitive Queue is Ending"),
                            },
                            Localized {
                                locale: String::from("es_ES"),
                                content: String::from("La cola competitiva se cerrará pronto."),
                            },
                            Localized {
                                locale: String::from("es_MX"),
                                content: String::from("El modo competitivo está por terminar"),
                            },
                            Localized {
                                locale: String::from("fr_FR"),
                                content: String::from("La file classée va bientôt fermer"),
                            },
                            Localized {
                                locale: String::from("id_ID"),
                                content: String::from("Antrean Kompetitif Akan Berakhir"),
                            },
                            Localized {
                                locale: String::from("it_IT"),
                                content: String::from("La coda competitiva sta per terminare"),
                            },
                            Localized {
                                locale: String::from("ja_JP"),
                                content: String::from("コンペティティブがまもなく終了"),
                            },
                            Localized {
                                locale: String::from("ko_KR"),
                                content: String::from("경쟁전 종료 예정"),
                            },
                            Localized {
                                locale: String::from("pl_PL"),
                                content: String::from("Kolejka rankingowa zostanie zamknięta"),
                            },
                            Localized {
                                locale: String::from("pt_BR"),
                                content: String::from("A fila do Competitivo vai fechar logo"),
                            },
                            Localized {
                                locale: String::from("ru_RU"),
                                content: String::from("Рейтинговые игры завершаются"),
                            },
                            Localized {
                                locale: String::from("th_TH"),
                                content: String::from("คิวการแข่งขันกำลังจะจบลง"),
                            },
                            Localized {
                                locale: String::from("tr_TR"),
                                content: String::from("Rekabet Modu Sona Eriyor"),
                            },
                            Localized {
                                locale: String::from("vi_VN"),
                                content: String::from("Hàng Chờ Đấu Xếp Hạng sắp kết thúc"),
                            },
                            Localized {
                                locale: String::from("zh_TW"),
                                content: String::from("競技模式即將關閉"),
                            },
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 14182,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2024, 3, 1, 22, 37, 43)
                                    .unwrap()
                                    .with_nanosecond(153_783_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2024, 3, 1, 22, 37, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("game"),
                                    String::from("riotclient"),
                                    String::from("riotstatus"),
                                ],
                                translations: vec![
                                    Localized {
                                        locale: String::from("en_US"),
                                        content: String::from("This Act's Competitive queue is ending soon. Check your Act Rank tab for specific times."),
                                    },
                                    Localized {
                                        locale: String::from("ar_AE"),
                                        content: String::from("سينتهي طابور اللعب التنافسي لهذا المشهد قريبًا. راجع نافذة تصنيف المشهد الخاص بك للاطلاع على المواعيد المحددة."),
                                    },
                                    Localized {
                                        locale: String::from("de_DE"),
                                        content: String::from("Die gewertete Warteschlange dieses Akts endet bald. Schau im Tab „Akt-Rang“ die genauen Termine nach."),
                                    },
                                    Localized {
                                        locale: String::from("en_SG"),
                                        content: String::from("This Act's Competitive queue is ending soon. Check your Act Rank tab for specific times."),
                                    },
                                    Localized {
                                        locale: String::from("es_ES"),
                                        content: String::from("La cola competitiva de este acto se cerrará pronto. Echa un vistazo a la pestaña del rango de acto para conocer la fecha exacta."),
                                    },
                                    Localized {
                                        locale: String::from("es_MX"),
                                        content: String::from("El modo competitivo de este acto está por terminar. Revisa la pestaña de rango de acto para consultar el horario."),
                                    },
                                    Localized {
                                        locale: String::from("fr_FR"),
                                        content: String::from("La file classée de l'acte en cours sera bientôt close. Consultez votre onglet Rang d'acte pour connaître l'heure exacte de fermeture."),
                                    },
                                    Localized {
                                        locale: String::from("id_ID"),
                                        content: String::from("Antrean Competitive untuk Act ini akan segera berakhir. Periksa tab Rank Act kamu untuk mengetahui waktu tepatnya."),
                                    },
                                    Localized {
                                        locale: String::from("it_IT"),
                                        content: String::from("La coda competitiva di quest'Atto sta per terminare. Consulta la scheda Grado Atto della tua Carriera per conoscere le date precise."),
                                    },
                                    Localized {
                                        locale: String::from("ja_JP"),
                                        content: String::from("現 Act のコンペティティブがまもなく終了します。詳細な日程は「Act ランク」タブからご確認ください。"),
                                    },
                                    Localized {
                                        locale: String::from("ko_KR"),
                                        content: String::from("이번 액트의 경쟁전이 곧 종료됩니다. 정확한 시간은 액트 랭크 탭에서 확인할 수 있습니다."),
                                    },
                                    Localized {
                                        locale: String::from("pl_PL"),
                                        content: String::from("Kolejka rankingowa tego aktu wkrótce zostanie zamknięta. Zajrzyj do zakładki z rangą aktu, aby poznać konkretne terminy."),
                                    },
                                    Localized {
                                        locale: String::from("pt_BR"),
                                        content: String::from("A fila do Competitivo deste Ato vai fechar em breve. Verifique os horários específicos na aba do seu Ranque de Ato."),
                                    },
                                    Localized {
                                        locale: String::from("ru_RU"),
                                        content: String::from("Рейтинговые игры этого акта близятся к завершению. Точное время окончания можно посмотреть в ранге акта."),
                                    },
                                    Localized {
                                        locale: String::from("th_TH"),
                                        content: String::from("คิวการแข่งขันจัดอันดับของ Act นี้กำลังจะจบลง ตรวจสอบแถบแรงค์ Act สำหรับเวลาที่แน่นอน"),
                                    },
                                    Localized {
                                        locale: String::from("tr_TR"),
                                        content: String::from("Bu kısımdaki rekabet modu yakında sona eriyor. Tam tarihleri görmek için Kısım Kademesi sekmesine göz at."),
                                    },
                                    Localized {
                                        locale: String::from("vi_VN"),
                                        content: String::from("Hàng chờ Đấu Xếp Hạng của Phần này sắp kết thúc. Hãy xem trong mục Xếp Hạng Phần của bạn để biết thời gian cụ thể."),
                                    },
                                    Localized {
                                        locale: String::from("zh_TW"),
                                        content: String::from("此章的競技模式即將關閉，請至章牌階頁面查看確切結束時間。"),
                                    },
                                ],
                            },
                        ],
                    },
                ],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_maintenance() {
    let json = read_resource("v1-status/ok_maintenance.json");

    let expected = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V1StatusData {
                maintenances: vec![
                    Status {
                        id: 8183,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2024, 3, 5, 14, 25, 07)
                            .unwrap()
                            .with_nanosecond(688_150_000)
                            .unwrap(),
                        updated: None,
                        archived: None,
                        incident_severity: None,
                        maintenance_status: Some(String::from("scheduled")),
                        titles: vec![
                            Localized {
                                locale: String::from("en_US"),
                                content: String::from("Queues Disabled for Maintenance"),
                            },
                            Localized {
                                locale: String::from("ar_AE"),
                                content: String::from("الطوابير معطلة لإجراء الصيانة"),
                            },
                            Localized {
                                locale: String::from("de_DE"),
                                content: String::from("Spiel aufgrund von Wartungsarbeiten deaktiviert"),
                            },
                            Localized {
                                locale: String::from("en_SG"),
                                content: String::from("Queues Disabled for Maintenance"),
                            },
                            Localized {
                                locale: String::from("es_ES"),
                                content: String::from("Colas desactivadas por mantenimiento"),
                            },
                            Localized {
                                locale: String::from("es_MX"),
                                content: String::from("Colas inhabilitadas debido a mantenimiento"),
                            },
                            Localized {
                                locale: String::from("fr_FR"),
                                content: String::from("Files désactivées pour maintenance"),
                            },
                            Localized {
                                locale: String::from("id_ID"),
                                content: String::from("Antrean Dinonaktifkan untuk maintenance"),
                            },
                            Localized {
                                locale: String::from("it_IT"),
                                content: String::from("Code disattivate per manutenzione"),
                            },
                            Localized {
                                locale: String::from("ja_JP"),
                                content: String::from("キューは現在無効化されています"),
                            },
                            Localized {
                                locale: String::from("ko_KR"),
                                content: String::from("점검으로 인한 대전 비활성화"),
                            },
                            Localized {
                                locale: String::from("pl_PL"),
                                content: String::from("Kolejki wyłączone z powodu przerwy technicznej"),
                            },
                            Localized {
                                locale: String::from("pt_BR"),
                                content: String::from("Filas desativadas para manutenção"),
                            },
                            Localized {
                                locale: String::from("ru_RU"),
                                content: String::from("Очереди временно отключены"),
                            },
                            Localized {
                                locale: String::from("th_TH"),
                                content: String::from("ไม่สามารถใช้งานคิวได้เนื่องจากการปิดปรับปรุงระบบ"),
                            },
                            Localized {
                                locale: String::from("tr_TR"),
                                content: String::from("Sıralar Bakım Çalışması Nedeniyle Devre Dışı"),
                            },
                            Localized {
                                locale: String::from("vi_VN"),
                                content: String::from("Hàng Chờ Bị Vô Hiệu Hóa Để Bảo Trì"),
                            },
                            Localized {
                                locale: String::from("zh_TW"),
                                content: String::from("列隊功能維護中"),
                            },
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 14267,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2024, 3, 5, 14, 25, 7)
                                    .unwrap()
                                    .with_nanosecond(750_298_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2024, 3, 6, 3, 30, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("game"),
                                    String::from("riotclient"),
                                    String::from("riotstatus"),
                                ],
                                translations: vec![
                                    Localized {
                                        locale: String::from("en_US"),
                                        content: String::from("Queues are temporarily unavailable while we perform scheduled maintenance."),
                                    },
                                    Localized {
                                        locale: String::from("ar_AE"),
                                        content: String::from("الطوابير غير متاحة مؤقتًا ريثما نجري عملية صيانة مقررة."),
                                    },
                                    Localized {
                                        locale: String::from("de_DE"),
                                        content: String::from("Aufgrund von geplanten Wartungsarbeiten ist das Spiel vorübergehend nicht verfügbar."),
                                    },
                                    Localized {
                                        locale: String::from("en_SG"),
                                        content: String::from("Queues are temporarily unavailable while we perform scheduled maintenance."),
                                    },
                                    Localized {
                                        locale: String::from("es_ES"),
                                        content: String::from("Las colas están desactivadas temporalmente mientras realizamos tareas de mantenimiento."),
                                    },
                                    Localized {
                                        locale: String::from("es_MX"),
                                        content: String::from("Las colas estarán temporalmente fuera de servicio mientras llevamos a cabo las tareas de mantenimiento programado."),
                                    },
                                    Localized {
                                        locale: String::from("fr_FR"),
                                        content: String::from("Les files d'attente sont temporairement indisponibles pendant que nous effectuons une maintenance programmée."),
                                    },
                                    Localized {
                                        locale: String::from("id_ID"),
                                        content: String::from("Antrean tak tersedia sementara selagi kami melakukan maintenance terjadwal."),
                                    },
                                    Localized {
                                        locale: String::from("it_IT"),
                                        content: String::from("Durante gli interventi di manutenzione programmati, le code saranno temporaneamente disabilitate."),
                                    },
                                    Localized {
                                        locale: String::from("ja_JP"),
                                        content: String::from("定期メンテナンス中のため、現在キューはご利用になれません。"),
                                    },
                                    Localized {
                                        locale: String::from("ko_KR"),
                                        content: String::from("정기 점검으로 인해 대전이 일시적으로 비활성화되었습니다. 양해 부탁드립니다."),
                                    },
                                    Localized {
                                        locale: String::from("pl_PL"),
                                        content: String::from("Kolejki są tymczasowo niedostępne z powodu zaplanowanej przerwy technicznej."),
                                    },
                                    Localized {
                                        locale: String::from("pt_BR"),
                                        content: String::from("As filas estão temporariamente indisponíveis enquanto realizamos uma manutenção agendada."),
                                    },
                                    Localized {
                                        locale: String::from("ru_RU"),
                                        content: String::from("Очереди временно отключены, пока мы проводим плановое техническое обслуживание."),
                                    },
                                    Localized {
                                        locale: String::from("th_TH"),
                                        content: String::from("คิวจับคู่ไม่สามารถใช้งานได้ชั่วคราวในระหว่างที่เราทำการปิดปรับปรุงระบบ"),
                                    },
                                    Localized {
                                        locale: String::from("tr_TR"),
                                        content: String::from("Planlı bakım çalışması süresince sıralar geçici olarak devre dışı bırakıldı."),
                                    },
                                    Localized {
                                        locale: String::from("vi_VN"),
                                        content: String::from("Hàng chờ tạm thời không khả dụng trong khi chúng tôi thực hiện bảo trì theo kế hoạch."),
                                    },
                                    Localized {
                                        locale: String::from("zh_TW"),
                                        content: String::from("我們正在進行例行維護，列隊功能將暫時關閉。"),
                                    },
                                ],
                            },
                        ],
                    }
                ],
                incidents: vec![
                    Status {
                        id: 8082,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2024, 2, 21, 16, 21, 56)
                            .unwrap()
                            .with_nanosecond(676_828_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2024, 2, 26, 0, 27, 9)
                                .unwrap()
                                .with_nanosecond(462_588_000)
                                .unwrap()
                        ),
                        archived: None,
                        incident_severity: Some(String::from("info")),
                        maintenance_status: None,
                        titles: vec![
                            Localized {
                                locale: String::from("en_US"),
                                content: String::from("Payment Issues"),
                            },
                            Localized {
                                locale: String::from("ru_RU"),
                                content: String::from("Проблемы с оплатой\r\n"),
                            },
                        ],
                        platforms: vec![
                            String::from("windows"),
                            String::from("macos"),
                            String::from("android"),
                            String::from("ios"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 14057,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2024, 2, 21, 16, 21, 56)
                                    .unwrap()
                                    .with_nanosecond(683_227_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2024, 2, 21, 16, 21, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("game"),
                                    String::from("riotclient"),
                                    String::from("riotstatus"),
                                ],
                                translations: vec![
                                    Localized {
                                        locale: String::from("en_US"),
                                        content: String::from("We're aware of a problem that's causing VP purchases to fail and are working on a fix."),
                                    },
                                    Localized {
                                        locale: String::from("ru_RU"),
                                        content: String::from("Мы знаем о проблеме, из-за которой не удается приобрести VP, и работаем над ее решением."),
                                    },
                                ],
                            },
                        ],
                    },
                ],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

use rusqlite::Connection;

pub fn seed_data(conn: &Connection) -> Result<(), rusqlite::Error> {
    let user_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
        .unwrap_or(0);

    if user_count > 0 {
        return Ok(());
    }

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let admin_hash =
        bcrypt::hash("admin123", bcrypt::DEFAULT_COST).expect("hash admin password");
    let user_hash =
        bcrypt::hash("123456", bcrypt::DEFAULT_COST).expect("hash user password");

    conn.execute(
        "INSERT INTO users (username, password_hash, role, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["admin", &admin_hash, "admin", &now],
    )?;

    conn.execute(
        "INSERT INTO users (username, password_hash, role, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["zhangsan", &user_hash, "user", &now],
    )?;

    conn.execute(
        "INSERT INTO users (username, password_hash, role, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["lisi", &user_hash, "user", &now],
    )?;

    conn.execute(
        "INSERT INTO suppliers (name, contact, phone, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["北京钢铁供应有限公司", "王建国", "010-88886666", "北京市朝阳区建国路88号", &now],
    )?;

    conn.execute(
        "INSERT INTO suppliers (name, contact, phone, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["上海精密零部件制造厂", "李明辉", "021-66668888", "上海市浦东新区张江路128号", &now],
    )?;

    conn.execute(
        "INSERT INTO suppliers (name, contact, phone, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["深圳电子元器件供应商", "陈志强", "0755-22223333", "深圳市南山区科技园路56号", &now],
    )?;

    conn.execute(
        "INSERT INTO suppliers (name, contact, phone, address, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["广州密封件贸易有限公司", "赵伟", "020-33334444", "广州市天河区天河路200号", &now],
    )?;

    let materials = vec![
        ("轴承6205-2RS", "轴承类", "25×52×15mm", "个", 500.0, 100.0, 28.50, 1),
        ("螺栓M10×50", "紧固件", "M10×50 8.8级", "个", 2000.0, 500.0, 1.20, 1),
        ("不锈钢板304-2mm", "板材", "1220×2440×2mm", "张", 50.0, 10.0, 380.00, 1),
        ("电缆RVV3×2.5", "线缆类", "3×2.5mm²", "米", 800.0, 200.0, 8.50, 3),
        ("密封圈DN50", "密封件", "DN50 丁腈橡胶", "个", 300.0, 80.0, 5.60, 4),
        ("减速机RV063", "传动件", "速比1:30", "台", 12.0, 3.0, 2680.00, 2),
        ("接触器CJX2-25", "电气件", "AC220V 25A", "个", 40.0, 10.0, 85.00, 3),
        ("气缸SC63×200", "气动件", "缸径63 行程200", "个", 25.0, 5.0, 320.00, 2),
        ("滤芯HX-40×10", "过滤件", "精度10μm", "个", 60.0, 15.0, 45.00, 4),
        ("链条08B-1", "传动件", "节距12.7mm 单排", "米", 100.0, 30.0, 35.00, 2),
        ("继电器MY2NJ", "电气件", "DC24V 8脚", "个", 80.0, 20.0, 18.50, 3),
        ("铜管T2Φ12", "管材", "外径12mm 壁厚1mm", "米", 150.0, 40.0, 22.00, 1),
    ];

    for (name, category, spec, unit, stock, min_stock, price, supplier_id) in &materials {
        conn.execute(
            "INSERT INTO materials (name, category, specification, unit, stock_quantity, min_stock, price, supplier_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![name, category, spec, unit, stock, min_stock, price, supplier_id, &now, &now],
        )?;
    }

    let requisitions = vec![
        (1, 20.0, "生产一部", "张三", "设备日常维护更换", "approved"),
        (2, 100.0, "生产二部", "李四", "产线紧固件补充", "approved"),
        (4, 50.0, "设备维修部", "王五", "车间线路改造", "pending"),
        (5, 30.0, "生产一部", "张三", "液压系统密封更换", "approved"),
        (7, 5.0, "电气维修部", "赵六", "配电柜维护", "pending"),
    ];

    for (material_id, quantity, department, applicant, purpose, status) in &requisitions {
        conn.execute(
            "INSERT INTO requisitions (material_id, quantity, department, applicant, purpose, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![material_id, quantity, department, applicant, purpose, status, &now],
        )?;
    }

    let scraps = vec![
        (3, 2.0, "切割余料，尺寸不足无法再利用", "王五", "approved"),
        (4, 30.0, "线路老化，绝缘层破损", "赵六", "approved"),
        (10, 5.0, "链条磨损伸长超标", "张三", "pending"),
    ];

    for (material_id, quantity, reason, handler, status) in &scraps {
        conn.execute(
            "INSERT INTO scraps (material_id, quantity, reason, handler, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![material_id, quantity, reason, handler, status, &now],
        )?;
    }

    Ok(())
}

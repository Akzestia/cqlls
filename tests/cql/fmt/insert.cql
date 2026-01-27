use "japanese";

INSERT INTO users (
    ユーザーID,
    ユーザー名,
    連絡先,
    住所一覧,
    タグ,
    設定,
    最終ログイン,
    作成日時
)
VALUES (
    uuid(),
    '不思議の国のアリス',
    {
        メール: 'アリス@example.com',
        電話番号: '+81-90-1234-5678',
        希望連絡時間: {
            '朝',
            '夕方'
        }
    },
    {
        '自宅': {
            通り: '東京都渋谷区神宮前1-2-3',
            市区町村: '東京',
            郵便番号: '150-0001',
            国: '日本'
        },
        '職場': {
            通り: '東京都千代田区丸の内4-5-6',
            市区町村: '東京',
            郵便番号: '100-0005',
            国: '日本'
        }
    },
    {
        'プレミアム',
        '認証済み',
        '早期採用者'
    },
    {
        'テーマ': 'ダーク',
        '言語': '日本語',
        '通知': '有効'
    },
    toTimestamp(now()),
    toTimestamp(now())
);

-- Insert users with UDT, SET, and MAP
INSERT INTO users (
    user_id,
    username,
    contact,
    addresses,
    tags,
    preferences,
    last_login,
    created_at
)
VALUES (
    uuid(),
    '不思議の国のアリス',
    {
        email: 'アリス@example.com',
        phone: '+81-90-1234-5678',
        preferred_time: {
            '朝',
            '夕方'
        }
    },
    {
        '自宅': {
            street: '東京都渋谷区神宮前1-2-3',
            city: '東京',
            zip_code: '150-0001',
            country: '日本'
        },
        '職場': {
            street: '東京都千代田区丸の内4-5-6',
            city: '東京',
            zip_code: '100-0005',
            country: '日本'
        }
    },
    {
        'プレミアム',
        '認証済み',
        '早期採用者'
    },
    {
        'theme': 'ダーク',
        'language': '日本語',
        'notifications': '有効'
    },
    toTimestamp(now()),
    toTimestamp(now())
);

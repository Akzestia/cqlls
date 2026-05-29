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
  'bob_builder',
  {
    email: 'bob@example.com',
    phone: '+1-555-0202',
    preferred_time: {
      'morning'
    }
  },
  {
    'home': {
      street: '789 Oak Ave',
      city: 'Austin',
      zip_code: '73301',
      country: 'USA'
    }
  },
  {
    'verified',
    'beta-tester'
  },
  {
    'theme': 'light',
    'language': 'en',
    'notifications': 'disabled'
  },
  toTimestamp(now()),
  toTimestamp(now())
);

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
  'charlie_brown',
  {
    email: 'charlie@example.com',
    phone: '+1-555-0303',
    preferred_time: {
      'evening',
      'night'
    }
  },
  {
    'work': {
      street: '101 Pine Rd',
      city: 'Seattle',
      zip_code: '98101',
      country: 'USA'
    }
  },
  {
    'premium'
  },
  {
    'theme': 'dark',
    'language': 'en',
    'notifications': 'enabled'
  },
  toTimestamp(now()),
  toTimestamp(now())
);

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
  'diana_prince',
  {
    email: 'diana@example.com',
    phone: '+1-555-0404',
    preferred_time: {
      'afternoon'
    }
  },
  {
    'home': {
      street: '555 Gateway Blvd',
      city: 'New York',
      zip_code: '10001',
      country: 'USA'
    },
    'work': {
      street: '777 Justice Way',
      city: 'Washington',
      zip_code: '20001',
      country: 'USA'
    }
  },
  {
    'verified',
    'moderator',
    'staff'
  },
  {
    'theme': 'dark',
    'language': 'es',
    'notifications': 'enabled'
  },
  toTimestamp(now()),
  toTimestamp(now())
);

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
  'ethan_hunt',
  {
    email: 'ethan@example.com',
    phone: '+1-555-0505',
    preferred_time: {
      'night'
    }
  },
  {
    'home': {
      street: '999 Secret Ln',
      city: 'Miami',
      zip_code: '33101',
      country: 'USA'
    }
  },
  {
    'early-adopter'
  },
  {
    'theme': 'light',
    'language': 'fr',
    'notifications': 'disabled'
  },
  toTimestamp(now()),
  toTimestamp(now())
);

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
  'fiona_shrek',
  {
    email: 'fiona@example.com',
    phone: '+1-555-0606',
    preferred_time: {
      'morning',
      'afternoon'
    }
  },
  {
    'home': {
      street: '42 Swamp Rd',
      city: 'Duloc',
      zip_code: '12345',
      country: 'FarAway'
    }
  },
  {
    'premium',
    'verified'
  },
  {
    'theme': 'dark',
    'language': 'en',
    'notifications': 'enabled'
  },
  toTimestamp(now()),
  toTimestamp(now())
);

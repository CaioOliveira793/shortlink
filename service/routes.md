# Routes

## user management

type User = {
	id: string,
	created: timestamp,
	updated: timestamp,
	email: string,
	username: string,
}

POST /user -> User
{
	username: string,
	email: string,
	password: string
}

PUT /user/password -> User
Authentication: Bearer {token}
Authentication-Password: string
Authentication-OTP: string
{
	password: string
}

DELETE /user
Authentication: Bearer {token}
Authentication-Password: string
Authentication-OTP: string

## authentication

POST /auth -> { user: User, token: string }
{
	email: string
	password: string
}

## url management

type ShortUrlEntry = {
	slug: string,
	created: timestamp,
	updated: timestamp,
	long_url: string,
	expires: timestamp | null,
	short_url: string,
	active: boolean,
}

POST /url
Authentication: Bearer {token}
{
	long_url: string,
	expires: timestamp | null,
	short_url: string | null
}

PUT /url/:short_url/expires
Authentication: Bearer {token}
{
	expires: timestamp | null
}

PUT /url/:short_url/active
Authentication: Bearer {token}
{
	active: boolean
}

DELETE /url/:short_url
Authentication: Bearer {token}

GET /url/:short_url -> ShortUrlEntry
Authentication: Bearer {token}

GET /url -> [ShortUrlEntry]
Authentication: Bearer {token}

## open

GET /:short_url

## usage

TODO: design usage collection routes and service

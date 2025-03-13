export interface RegistrationBody {
  email: string,
  name: string,
  password: string
}

export interface LoginBody {
  email: string,
  password: string
}

export interface User {
  email: string,
  name: string,
  isSuperuser: string
}

export interface AppStore {
  healthCheckTimeout: number,
  authenticated: boolean,
  sessionKey: string | null,
  authenticatedUser: User | null,
}

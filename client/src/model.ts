export interface RegistrationBody {
  email: string;
  name: string;
  password: string;
}

export interface LoginBody {
  email: string;
  password: string;
}

export interface User {
  email: string;
  name: string;
  isSuperuser: string;
}

export interface Event {
  id: number | null;
  startDate: Date;
  endDate: Date;
  startTime: Date;
  endTime: Date;
  place: string;
}

/*
 * Data store type for this application. Not to be confused with an online store
 * which allows users to download apps.
 */
export interface AppStore {
  healthCheckTimeout: number;
  authenticated: boolean;
  sessionKey: string | null;
  authenticatedUser: User | null;
  ownedEvents: Event[] | null;
}

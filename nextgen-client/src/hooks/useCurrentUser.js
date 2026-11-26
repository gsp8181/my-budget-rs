import { useEffect, useState } from 'react';
import api from '../api';

const PLACEHOLDER = { username: 'Loading...', email: 'Loading...' };

// The signed-in user's name and email from /api/me, shown in the side menus.
// Keeps the placeholder if the request fails.
export default function useCurrentUser() {
  const [user, setUser] = useState(PLACEHOLDER);

  useEffect(() => {
    let active = true;
    api.get('/me')
      .then((data) => {
        if (active && data) {
          setUser({
            username: data.username || PLACEHOLDER.username,
            email: data.email || PLACEHOLDER.email,
          });
        }
      })
      .catch(() => {});
    return () => { active = false; };
  }, []);

  return user;
}

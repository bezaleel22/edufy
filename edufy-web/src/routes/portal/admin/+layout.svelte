<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import { goto } from '$app/navigation';

  interface User {
    id: string;
    email: string;
    role: string;
    full_name: string;
    expires_at?: number;
  }

  let user: User | null = null;
  let loading = true;

  onMount(async () => {
    if (!browser) return;

    // Quick client-side auth check first
    const userData = localStorage.getItem('user');
    if (!userData) {
      goto('/login');
      return;
    }

    const parsedUser = JSON.parse(userData);
    
    // Check if session expired
    if (Date.now() > parsedUser.expires_at) {
      localStorage.removeItem('user');
      goto('/login');
      return;
    }

    // Check admin role
    if (parsedUser.role !== 'admin') {
      goto('/portal'); // Redirect to regular portal
      return;
    }

    // Verify with API using HttpOnly cookie
    try {
      const response = await fetch('https://school.llacademy.ng/api/users/me', {
        credentials: 'include' // Sends HttpOnly cookie
      });

      if (response.ok) {
        const apiUser = await response.json();
        user = apiUser;
        loading = false;
      } else {
        // Session invalid, clear and redirect
        localStorage.removeItem('user');
        goto('/login');
      }
    } catch (error) {
      console.error('Auth verification failed:', error);
      localStorage.removeItem('user');
      goto('/login');
    }
  });

  async function logout() {
    localStorage.removeItem('user');
    
    try {
      await fetch('https://school.llacademy.ng/api/auth/logout', {
        method: 'POST',
        credentials: 'include'
      });
    } catch (error) {
      console.error('Logout API failed:', error);
    }
    
    goto('/login');
  }
</script>

{#if loading}
  <div class="loading">
    <p>Verifying authentication...</p>
  </div>
{:else if user}
  <div class="admin-layout">
    <header>
      <h1>Admin Dashboard</h1>
      <div class="user-info">
        <span>Welcome, {user.full_name}</span>
        <button on:click={logout}>Logout</button>
      </div>
    </header>
    
    <nav>
      <a href="/portal/admin">Dashboard</a>
      <a href="/portal/admin/posts">Manage Posts</a>
      <a href="/portal/admin/users">Manage Users</a>
    </nav>
    
    <main>
      <slot />
    </main>
  </div>
{/if}

<style>
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }
  
  .admin-layout {
    min-height: 100vh;
  }
  
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
  }
  
  nav {
    padding: 1rem 2rem;
    background: #e9ecef;
  }
  
  nav a {
    margin-right: 1rem;
    text-decoration: none;
    color: #495057;
  }
  
  nav a:hover {
    color: #007bff;
  }
  
  main {
    padding: 2rem;
  }
  
  button {
    background: #dc3545;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }
</style>

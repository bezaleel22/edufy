<script lang="ts">
  import { goto } from '$app/navigation';
  
  export let data: any;
  
  let loading = false;
  let error = '';
  let email = '';
  let password = '';
  
  const { redirectUrl } = data;
  
  async function handleLogin(event: Event) {
    event.preventDefault();
    loading = true;
    error = '';
    
    if (!email || !password) {
      error = 'Email and password are required';
      loading = false;
      return;
    }
    
    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, password }),
        credentials: 'include'
      });
      
      if (response.ok) {
        // Login successful, redirect to intended page
        await goto(redirectUrl);
      } else if (response.status === 401) {
        error = 'Invalid email or password';
      } else {
        error = 'Login service unavailable';
      }
    } catch (err) {
      console.error('Login error:', err);
      error = 'Login service unavailable';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Login - LLA Web</title>
</svelte:head>

<div class="login-container">
  <div class="login-card">
    <div class="login-header">
      <h1>LLA Web Login</h1>
      <p>Sign in to access your portal</p>
    </div>

    <form
      class="login-form"
      on:submit={handleLogin}
    >
      <div class="form-group">
        <label for="email">Email</label>
        <input
          type="email"
          id="email"
          bind:value={email}
          required
          disabled={loading}
          placeholder="Enter your email"
        />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input
          type="password"
          id="password"
          bind:value={password}
          required
          disabled={loading}
          placeholder="Enter your password"
        />
      </div>

      {#if error}
        <div class="error-message">
          {error}
        </div>
      {/if}

      <button type="submit" class="login-btn" disabled={loading}>
        {loading ? "Signing in..." : "Sign In"}
      </button>
    </form>

    <div class="login-footer">
      <p>Don't have an account? Contact your administrator.</p>
    </div>
  </div>
</div>

<style>
  .login-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 1rem;
  }

  .login-card {
    background: white;
    border-radius: 8px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    width: 100%;
    max-width: 400px;
    overflow: hidden;
  }

  .login-header {
    background: #2c3e50;
    color: white;
    padding: 2rem;
    text-align: center;
  }

  .login-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 1.75rem;
  }

  .login-header p {
    margin: 0;
    opacity: 0.9;
  }

  .login-form {
    padding: 2rem;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #2c3e50;
  }

  .form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
    box-sizing: border-box;
  }

  .form-group input:focus {
    outline: none;
    border-color: #3498db;
    box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.2);
  }

  .form-group input:disabled {
    background-color: #f8f9fa;
    cursor: not-allowed;
  }

  .error-message {
    background-color: #f8d7da;
    color: #721c24;
    padding: 0.75rem;
    border-radius: 4px;
    margin-bottom: 1rem;
    border: 1px solid #f5c6cb;
  }

  .login-btn {
    width: 100%;
    padding: 0.75rem;
    background: #3498db;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .login-btn:hover:not(:disabled) {
    background: #2980b9;
  }

  .login-btn:disabled {
    background: #bdc3c7;
    cursor: not-allowed;
  }

  .login-footer {
    background: #ecf0f1;
    padding: 1rem 2rem;
    text-align: center;
    border-top: 1px solid #ddd;
  }

  .login-footer p {
    margin: 0;
    color: #7f8c8d;
    font-size: 0.9rem;
  }
</style>

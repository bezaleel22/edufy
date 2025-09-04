<script lang="ts">
  export let data: any;
  
  const { user } = data;
</script>

<div class="portal-layout">
  <header class="portal-header">
    <nav class="portal-nav">
      <div class="nav-brand">
        <h1>LLA Portal</h1>
      </div>
      <div class="nav-user">
        <span>Welcome, {user.full_name}</span>
        <form action="/api/auth/logout" method="post" style="display: inline;">
          <button type="submit" class="logout-btn">Logout</button>
        </form>
      </div>
    </nav>
  </header>
  
  <main class="portal-main">
    <aside class="portal-sidebar">
      <ul class="sidebar-menu">
        <li><a href="/portal" class="menu-item">Dashboard</a></li>
        <li><a href="/portal/profile" class="menu-item">Profile</a></li>
        {#if user.role === 'admin'}
          <li><a href="/portal/admin" class="menu-item">Admin</a></li>
          <li><a href="/portal/admin/posts" class="menu-item">Manage Posts</a></li>
          <li><a href="/portal/admin/users" class="menu-item">Manage Users</a></li>
        {/if}
      </ul>
    </aside>
    
    <div class="portal-content">
      <slot />
    </div>
  </main>
</div>

<style>
  .portal-layout {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
  
  .portal-header {
    background-color: #2c3e50;
    color: white;
    padding: 1rem;
  }
  
  .portal-nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .nav-brand h1 {
    margin: 0;
    font-size: 1.5rem;
  }
  
  .nav-user {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .logout-btn {
    background-color: #e74c3c;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .logout-btn:hover {
    background-color: #c0392b;
  }
  
  .portal-main {
    flex: 1;
    display: flex;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
  }
  
  .portal-sidebar {
    width: 250px;
    background-color: #ecf0f1;
    padding: 2rem 0;
    min-height: calc(100vh - 80px);
  }
  
  .sidebar-menu {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  
  .sidebar-menu li {
    margin: 0;
  }
  
  .menu-item {
    display: block;
    padding: 1rem 2rem;
    color: #2c3e50;
    text-decoration: none;
    border-bottom: 1px solid #bdc3c7;
    transition: background-color 0.2s;
  }
  
  .menu-item:hover {
    background-color: #d5dbdb;
  }
  
  .portal-content {
    flex: 1;
    padding: 2rem;
    background-color: white;
  }
  
  @media (max-width: 768px) {
    .portal-main {
      flex-direction: column;
    }
    
    .portal-sidebar {
      width: 100%;
      min-height: auto;
    }
    
    .sidebar-menu {
      display: flex;
      overflow-x: auto;
    }
    
    .sidebar-menu li {
      flex-shrink: 0;
    }
    
    .menu-item {
      white-space: nowrap;
      border-bottom: none;
      border-right: 1px solid #bdc3c7;
    }
  }
</style>

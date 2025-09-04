<script lang="ts">
  export let data: any;
  
  const { user } = data;
</script>

<svelte:head>
  <title>Portal Dashboard - LLA Web</title>
</svelte:head>

<div class="dashboard">
  <h1>Dashboard</h1>
  
  <div class="dashboard-grid">
    <div class="dashboard-card">
      <h2>Welcome Back</h2>
      <p>Hello, {user.full_name}!</p>
      <p>You're logged in as: <strong>{user.role}</strong></p>
      <p>Email: {user.email}</p>
    </div>
    
    <div class="dashboard-card">
      <h2>Quick Actions</h2>
      <div class="action-buttons">
        <a href="/portal/profile" class="action-btn">Edit Profile</a>
        {#if user.role === 'admin'}
          <a href="/portal/admin/posts" class="action-btn">Manage Posts</a>
          <a href="/portal/admin/posts/new" class="action-btn primary">Create New Post</a>
        {/if}
      </div>
    </div>
    
    {#if user.role === 'admin'}
      <div class="dashboard-card">
        <h2>Admin Overview</h2>
        <p>As an administrator, you have access to:</p>
        <ul>
          <li>Blog post management</li>
          <li>User management</li>
          <li>System configuration</li>
        </ul>
      </div>
    {/if}
    
    <div class="dashboard-card">
      <h2>Recent Activity</h2>
      <p>Your recent activity will be displayed here.</p>
    </div>
  </div>
</div>

<style>
  .dashboard {
    padding: 0;
  }
  
  .dashboard h1 {
    margin-bottom: 2rem;
    color: #2c3e50;
  }
  
  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .dashboard-card {
    background: white;
    border: 1px solid #e1e8ed;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .dashboard-card h2 {
    margin-top: 0;
    margin-bottom: 1rem;
    color: #2c3e50;
    font-size: 1.25rem;
  }
  
  .dashboard-card p {
    margin-bottom: 0.5rem;
    color: #555;
  }
  
  .dashboard-card ul {
    margin: 0.5rem 0;
    padding-left: 1.5rem;
  }
  
  .dashboard-card li {
    margin-bottom: 0.25rem;
    color: #555;
  }
  
  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .action-btn {
    display: inline-block;
    padding: 0.75rem 1rem;
    background-color: #3498db;
    color: white;
    text-decoration: none;
    border-radius: 4px;
    text-align: center;
    transition: background-color 0.2s;
  }
  
  .action-btn:hover {
    background-color: #2980b9;
  }
  
  .action-btn.primary {
    background-color: #27ae60;
  }
  
  .action-btn.primary:hover {
    background-color: #219a52;
  }
  
  @media (max-width: 768px) {
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }
</style>

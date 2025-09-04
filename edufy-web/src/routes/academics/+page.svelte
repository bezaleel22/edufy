<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  
  onMount(() => {
    goto('/', { replaceState: true });
  });
</script>

<!-- This component will redirect to home page -->
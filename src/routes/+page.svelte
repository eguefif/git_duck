<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

    let repo = $state('');
    let commits = $state([]);
    let repoState = $state('');

  async function getRepoState(e) {
      if (repo.length == 0) {
          repo = "/home/eguefif/repo_test_git";
      }
      try {
        return await invoke('get_repo_state', { repo } )
      } catch(e) {
        console.log("Error: ", e);
      }

  }

  async function openRepo(e) {
      e.preventDefault()
      if (repo.length == 0) {
          repo = "/home/eguefif/repo_test_git";
      }
      commits = await invoke('list_commits_head', { repo });
      repoState = await getRepoState();
  }
</script>

<main class="container">
    <h3>List of Commits</h3>
    <form onsubmit={openRepo}>
        <input bind:value={repo} />
        <button type="submit">Open repo</button>
    </form>
    <p>State: {repoState}</p>
    <ul>
    {#if commits.length > 0 }
        {#each commits as [sha, name]}
            <li>{sha}:  {name}</li>
        {/each}
    {/if}
    </ul>
</main>

<style>
</style>


<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

    let repo = $state('');
    let commits = $state([]);
    let repo_name = $state('');

  async function openRepo(e) {
      e.preventDefault()
      if (repo.length == 0) {
          repo = "/home/eguefif/repo_test_git";
      }
      repo_name = repo;
      console.log(repo);
      commits = await invoke('list_commits_head', { repo });
      console.log(commits);

  }

</script>

<main class="container">
    <h3>List of Commits</h3>
    <form onsubmit={openRepo}>
        <input bind:value={repo} />
        <button type="submit">Open repo</button>
    </form>
    <p>Repo: {repo_name}</p>

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


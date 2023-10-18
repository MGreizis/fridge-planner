<script>
  import "../app.css";
</script>

<slot />

<style lang="postcss">
  :global(html) {
    font-family: "Inter", sans-serif;
    font-size: 16px;
    font-weight: 400;
    line-height: 1.5;
    color: #1f2937;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }
  :global(a) {
    color: #2ECC71;
    text-decoration: none;
    &:hover {
      text-decoration: underline;
    }
  }
</style>

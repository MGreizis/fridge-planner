<script>
  import "../app.css";
</script>

<slot />

<style lang="postcss">
  :global(html) {
    font-family:'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    font-size: 16px;
    font-weight: 400;
    line-height: 1.5;
    color: #1f2937;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    background: rgb(180, 180, 180);
  }
  :global(a) {
    color: rgb(22 163 74); /* green-600 in tailwind */
    text-decoration: none;
    &:hover {
      text-decoration: underline;
    }
  }
</style>

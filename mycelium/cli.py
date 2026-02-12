"""Mycelium CLI - Static analysis tool for mapping codebase connections."""

from __future__ import annotations

from pathlib import Path

import click

from mycelium.config import AnalysisConfig
from mycelium.output import write_output
from mycelium.pipeline import run_pipeline


@click.group()
def cli() -> None:
    """Mycelium - Map the hidden network of connections in your codebase."""
    pass


def _run_with_progress(config: AnalysisConfig):
    """Run the pipeline with Rich progress display."""
    from rich.console import Console
    from rich.progress import Progress, SpinnerColumn, TextColumn, TimeElapsedColumn
    from rich.table import Table

    console = Console()

    with Progress(
        SpinnerColumn(),
        TextColumn("[bold blue]{task.description}"),
        TimeElapsedColumn(),
        console=console,
        transient=True,
    ) as progress:
        task = progress.add_task("Initialising...", total=None)

        def on_phase(name, label):
            progress.update(task, description=label)

        result = run_pipeline(config, progress_callback=on_phase)

    # Summary table
    stats = result.stats
    timings = result.metadata.get("phase_timings", {})

    table = Table(title=f"Mycelium Analysis: {Path(config.repo_path).name}", show_edge=False)
    table.add_column("Metric", style="bold")
    table.add_column("Value", justify="right")

    table.add_row("Files", str(stats.get("files", 0)))
    table.add_row("Symbols", str(stats.get("symbols", 0)))
    table.add_row("Calls", str(stats.get("calls", 0)))
    table.add_row("Communities", str(stats.get("communities", 0)))
    table.add_row("Processes", str(stats.get("processes", 0)))

    langs = stats.get("languages", {})
    if langs:
        lang_str = ", ".join(f"{k}: {v}" for k, v in sorted(langs.items()))
        table.add_row("Languages", lang_str)

    duration = result.metadata.get("analysis_duration_ms", 0)
    table.add_row("Duration", f"{duration:.1f}ms")

    console.print(table)

    if config.verbose and timings:
        timing_table = Table(title="Phase Timings", show_edge=False)
        timing_table.add_column("Phase", style="bold")
        timing_table.add_column("Time (ms)", justify="right")
        for phase, ms in timings.items():
            timing_table.add_row(phase, f"{ms * 1000:.1f}")
        console.print(timing_table)

    return result


def _run_quiet(config: AnalysisConfig):
    """Run the pipeline with no output."""
    return run_pipeline(config)


@cli.command()
@click.argument("path", type=click.Path(exists=True))
@click.option("-o", "--output", "output_path", default=None, help="Output JSON file path")
@click.option("-l", "--languages", default=None, help="Comma-separated language filter")
@click.option("--resolution", default=1.0, type=float, help="Louvain resolution parameter")
@click.option("--max-processes", default=75, type=int, help="Maximum execution flows to detect")
@click.option("--max-depth", default=10, type=int, help="Maximum BFS trace depth")
@click.option("--exclude", multiple=True, help="Additional glob patterns to exclude")
@click.option("--verbose", is_flag=True, help="Show per-phase timing breakdown")
@click.option("--quiet", is_flag=True, help="Suppress all output except errors")
def analyze(
    path: str,
    output_path: str | None,
    languages: str | None,
    resolution: float,
    max_processes: int,
    max_depth: int,
    exclude: tuple[str, ...],
    verbose: bool,
    quiet: bool,
) -> None:
    """Analyse a source code repository and produce a structural map."""
    repo_path = Path(path).resolve()

    if output_path is None:
        output_path = f"{repo_path.name}.mycelium.json"

    lang_filter = None
    if languages:
        lang_filter = [l.strip() for l in languages.split(",")]

    config = AnalysisConfig(
        repo_path=str(repo_path),
        output_path=output_path,
        languages=lang_filter,
        resolution=resolution,
        max_processes=max_processes,
        max_depth=max_depth,
        exclude_patterns=list(exclude),
        verbose=verbose,
        quiet=quiet,
    )

    if quiet:
        result = _run_quiet(config)
    else:
        result = _run_with_progress(config)

    write_output(result, output_path)

    if not quiet:
        from rich.console import Console
        Console().print(f"[green]Output written to:[/green] {output_path}")


if __name__ == "__main__":
    cli()

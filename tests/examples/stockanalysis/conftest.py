"""Test fixtures for stockanalysis adapter tests."""

import pytest


@pytest.fixture
def stockanalysis_html_content() -> str:
    """Fixture for stockanalysis.com HTML table content.

    Simulates HTML table from:
    https://stockanalysis.com/stocks/tsla/history/
    """
    return """
    <table>
      <thead>
        <tr>
          <th>Date</th>
          <th>Open</th>
          <th>High</th>
          <th>Low</th>
          <th>Close</th>
          <th>Adj Close</th>
          <th>Change</th>
          <th>Volume</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Feb 2, 2026</td>
          <td>260.03</td>
          <td>270.49</td>
          <td>259.21</td>
          <td>269.96</td>
          <td>269.96</td>
          <td>4.04%</td>
          <td>73,368,699</td>
        </tr>
        <tr>
          <td>Feb 1, 2026</td>
          <td>255.00</td>
          <td>262.50</td>
          <td>254.00</td>
          <td>259.50</td>
          <td>259.50</td>
          <td>1.77%</td>
          <td>65,000,000</td>
        </tr>
      </tbody>
    </table>
    """


@pytest.fixture
def stockanalysis_html_empty() -> str:
    """Fixture for stockanalysis HTML table with header only (no data rows)."""
    return """
    <table>
      <thead>
        <tr>
          <th>Date</th>
          <th>Open</th>
          <th>High</th>
          <th>Low</th>
          <th>Close</th>
          <th>Adj Close</th>
          <th>Change</th>
          <th>Volume</th>
        </tr>
      </thead>
      <tbody>
      </tbody>
    </table>
    """


@pytest.fixture
def stockanalysis_html_invalid() -> str:
    """Fixture for malformed HTML (no table element)."""
    return "<div>No table here</div>"


@pytest.fixture
def stockanalysis_html_no_tbody() -> str:
    """Fixture for HTML table without tbody element."""
    return """
    <table>
      <thead>
        <tr>
          <th>Date</th>
          <th>Open</th>
          <th>High</th>
          <th>Low</th>
          <th>Close</th>
          <th>Adj Close</th>
          <th>Change</th>
          <th>Volume</th>
        </tr>
      </thead>
    </table>
    """


@pytest.fixture
def stockanalysis_row_valid() -> list[str]:
    """Fixture for valid HTML table row as list."""
    return [
        "Feb 2, 2026",
        "260.03",
        "270.49",
        "259.21",
        "269.96",
        "269.96",
        "4.04%",
        "73,368,699",
    ]


@pytest.fixture
def stockanalysis_row_insufficient() -> list[str]:
    """Fixture for HTML row with insufficient columns."""
    return ["Feb 2, 2026", "260.03", "270.49"]

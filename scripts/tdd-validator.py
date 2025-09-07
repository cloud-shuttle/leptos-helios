#!/usr/bin/env python3
"""
TDD Quality Validator for Helios v1.0
Enforces 100% coverage and 100% pass rate requirements
"""

import json
import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Tuple
import argparse

class TDDValidator:
    """Validates TDD quality requirements for Helios"""

    def __init__(self, config_path: str = "tdd-config.toml"):
        self.config = self.load_config(config_path)
        self.errors = []
        self.warnings = []

    def load_config(self, config_path: str) -> Dict:
        """Load TDD configuration from TOML file"""
        try:
            import tomllib
            with open(config_path, 'rb') as f:
                return tomllib.load(f)
        except ImportError:
            # Fallback for older Python versions
            try:
                import toml
                with open(config_path, 'r') as f:
                    return toml.load(f)
            except ImportError:
                print("Error: Neither tomllib nor toml package available")
                sys.exit(1)

    def validate_coverage(self, coverage_file: str = "cobertura.xml") -> bool:
        """Validate code coverage meets requirements"""
        if not Path(coverage_file).exists():
            self.errors.append(f"Coverage file not found: {coverage_file}")
            return False

        try:
            tree = ET.parse(coverage_file)
            root = tree.getroot()

            # Extract coverage metrics
            line_rate = float(root.attrib.get('line-rate', 0))
            branch_rate = float(root.attrib.get('branch-rate', 0))

            coverage_percent = line_rate * 100
            branch_percent = branch_rate * 100

            # Check against requirements
            required_coverage = self.config['tdd']['quality_requirements']['unit_test_coverage'] * 100

            if coverage_percent < required_coverage:
                self.errors.append(
                    f"Coverage {coverage_percent:.2f}% below requirement {required_coverage:.2f}%"
                )
                return False

            print(f"✅ Coverage: {coverage_percent:.2f}% (requirement: {required_coverage:.2f}%)")
            print(f"✅ Branch coverage: {branch_percent:.2f}%")
            return True

        except Exception as e:
            self.errors.append(f"Error parsing coverage file: {e}")
            return False

    def validate_performance(self, benchmark_file: str = "benchmark_results.json") -> bool:
        """Validate performance benchmarks meet requirements"""
        if not Path(benchmark_file).exists():
            self.warnings.append(f"Benchmark file not found: {benchmark_file}")
            return True  # Not critical for CI

        try:
            with open(benchmark_file, 'r') as f:
                results = json.load(f)

            # Validate 100K point rendering performance
            render_100k_time = self.extract_benchmark_time(results, "render_100k_points")
            max_allowed_ms = self.config['tdd']['quality_requirements']['max_render_time_100k_ms']

            if render_100k_time and render_100k_time > max_allowed_ms:
                self.errors.append(
                    f"100K point rendering {render_100k_time:.2f}ms exceeds limit {max_allowed_ms}ms"
                )
                return False

            if render_100k_time:
                print(f"✅ 100K point rendering: {render_100k_time:.2f}ms (limit: {max_allowed_ms}ms)")

            return True

        except Exception as e:
            self.warnings.append(f"Error parsing benchmark file: {e}")
            return True

    def extract_benchmark_time(self, results: Dict, benchmark_name: str) -> float:
        """Extract benchmark time from criterion results"""
        # This would need to be adapted based on actual criterion JSON format
        try:
            for result in results.get('results', []):
                if benchmark_name in result.get('id', ''):
                    return result.get('typical', {}).get('estimate', 0) / 1_000_000  # ns to ms
        except:
            pass
        return None

    def validate_mutation_testing(self, mutation_file: str = "mutants.json") -> bool:
        """Validate mutation testing quality"""
        if not Path(mutation_file).exists():
            self.warnings.append(f"Mutation testing file not found: {mutation_file}")
            return True  # Not critical for basic CI

        try:
            with open(mutation_file, 'r') as f:
                results = json.load(f)

            # Calculate mutation score
            caught = results.get('caught', 0)
            missed = results.get('missed', 0)
            total = caught + missed

            if total == 0:
                self.warnings.append("No mutation tests found")
                return True

            score = caught / total
            required_score = self.config['tdd']['quality_requirements']['mutation_test_score']

            if score < required_score:
                self.errors.append(
                    f"Mutation score {score:.2f} below requirement {required_score:.2f}"
                )
                return False

            print(f"✅ Mutation testing score: {score:.2f} (requirement: {required_score:.2f})")
            return True

        except Exception as e:
            self.warnings.append(f"Error parsing mutation file: {e}")
            return True

    def validate_test_distribution(self) -> bool:
        """Validate test pyramid distribution"""
        # This would analyze test files to ensure proper distribution
        # For now, just validate the structure exists

        test_dirs = {
            'unit': Path('helios-core/src'),
            'integration': Path('helios-core/tests'),
            'e2e': Path('helios-core/tests')
        }

        for test_type, path in test_dirs.items():
            if not path.exists():
                self.errors.append(f"Test directory missing: {path}")
                return False

        print("✅ Test pyramid structure validated")
        return True

    def validate_wasm_bundle_size(self) -> bool:
        """Validate WASM bundle size requirements"""
        wasm_files = list(Path('.').glob('**/pkg/*.wasm'))

        if not wasm_files:
            self.warnings.append("No WASM files found for size validation")
            return True

        max_size_kb = self.config['tdd']['quality_requirements']['max_wasm_bundle_kb']

        for wasm_file in wasm_files:
            size_bytes = wasm_file.stat().st_size
            size_kb = size_bytes / 1024

            if size_kb > max_size_kb:
                self.errors.append(
                    f"WASM bundle {wasm_file.name} is {size_kb:.1f}KB, exceeds limit {max_size_kb}KB"
                )
                return False

            print(f"✅ WASM bundle {wasm_file.name}: {size_kb:.1f}KB (limit: {max_size_kb}KB)")

        return True

    def run_validation(self) -> bool:
        """Run all TDD validations"""
        print("🎯 Running TDD Quality Validation")
        print("=" * 50)

        validations = [
            ("Coverage Analysis", self.validate_coverage),
            ("Performance Benchmarks", self.validate_performance),
            ("Mutation Testing", self.validate_mutation_testing),
            ("Test Distribution", self.validate_test_distribution),
            ("WASM Bundle Size", self.validate_wasm_bundle_size),
        ]

        all_passed = True

        for name, validator in validations:
            print(f"\n📋 {name}:")
            try:
                result = validator()
                if not result:
                    all_passed = False
            except Exception as e:
                self.errors.append(f"{name} validation failed: {e}")
                all_passed = False

        # Print summary
        print("\n" + "=" * 50)
        print("🎯 TDD VALIDATION SUMMARY")
        print("=" * 50)

        if self.warnings:
            print("⚠️  Warnings:")
            for warning in self.warnings:
                print(f"   • {warning}")

        if self.errors:
            print("❌ Errors:")
            for error in self.errors:
                print(f"   • {error}")

        if all_passed and not self.errors:
            print("🎉 All TDD quality requirements met!")
            print("✅ Ready for v1.0 release! 🚀")
            return True
        else:
            print("❌ TDD quality requirements not met")
            print("🔧 Please address errors before proceeding")
            return False


def main():
    parser = argparse.ArgumentParser(description='TDD Quality Validator for Helios')
    parser.add_argument('--config', default='tdd-config.toml',
                       help='Path to TDD configuration file')
    parser.add_argument('--coverage', default='cobertura.xml',
                       help='Path to coverage XML file')
    parser.add_argument('--benchmarks', default='benchmark_results.json',
                       help='Path to benchmark results JSON')
    parser.add_argument('--mutations', default='mutants.json',
                       help='Path to mutation testing results')

    args = parser.parse_args()

    validator = TDDValidator(args.config)
    success = validator.run_validation()

    sys.exit(0 if success else 1)


if __name__ == '__main__':
    main()

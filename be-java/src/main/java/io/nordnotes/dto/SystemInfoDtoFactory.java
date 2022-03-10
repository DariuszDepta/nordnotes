package io.nordnotes.dto;

import org.springframework.stereotype.Service;

/**
 * A factory for system information DTOs.
 */
@Service
public class SystemInfoDtoFactory {

  /**
   * Creates a DTO with system information.
   *
   * @param name      System name.
   * @param version   System version.
   * @param legalNote Legal note.
   * @return System information DTO.
   */
  public ResultDto<SystemInfoDto> from(String name, String version, String legalNote) {
    SystemInfoDto dto = new SystemInfoDto();
    dto.setName(name);
    dto.setVersion(version);
    dto.setLegalNote(legalNote);
    return new ResultDto<>(dto);
  }
}
